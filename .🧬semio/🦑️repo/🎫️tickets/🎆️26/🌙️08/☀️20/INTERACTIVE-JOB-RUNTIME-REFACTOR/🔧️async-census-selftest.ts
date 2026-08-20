#!/usr/bin/env bun
// 🔧️async-census-selftest.ts — TEMPORARY validation harness for 🔧️async-census.ts's parser
// primitives, run against synthetic Rust snippets (not the live tree). Ticket-folder scratch only.

import { cleanRustSource, findBodyOrDecl, scanBody } from "./🔧️async-census.ts";

const src = `
pub async fn real_suspend() {
    let x = some_channel.recv().await;
    println!("{}", x);
}

pub async fn calls_only_decorative() {
    decorative_one().await;
    decorative_two().await;
}

async fn decorative_one() { let mut s = 0; for i in 0..10 { s += i; } }
async fn decorative_two() { let mut s = 0; while s < 5 { s += 1; } }

pub async fn has_nested_fn_helper() {
    async fn inner_helper() {
        some_future.await;
    }
    let mut total = 0;
    for i in 0..3 { total += i; }
}

pub async fn closure_await() {
    let f = || async {
        real_suspend().await;
    };
    f().await;
}

pub async fn raw_string_trap() {
    let s = r#"this has a fake .await and "quotes" and { braces } inside"#;
    let s2 = "escaped \\" quote and { brace and .await token";
    let c = '\\'';
    // comment with .await and { unbalanced brace
    /* block comment with { nested /* deeper */ } .await */
    let x = 5;
}

trait Foo {
    async fn declared_only(&self);
}
`;

const clean = cleanRustSource(src);
let pass = 0, fail = 0;
function check(label: string, cond: boolean) {
  if (cond) { pass++; console.log(`PASS ${label}`); }
  else { fail++; console.log(`FAIL ${label}`); }
}

function findAndScan(name: string) {
  const idx = clean.indexOf(`fn ${name}`);
  check(`${name}: found in cleaned text`, idx >= 0);
  const nameEnd = idx + `fn ${name}`.length;
  const sig = findBodyOrDecl(clean, nameEnd);
  check(`${name}: signature scan succeeded`, !!sig);
  return sig;
}

// real_suspend: 1 own-level await
{
  const sig = findAndScan("real_suspend")!;
  check("real_suspend: has body", sig.hasBody);
  const body = scanBody(clean, sig.bodyStart!)!;
  check("real_suspend: own await count === 1", body.ownAwaitCount === 1);
}

// calls_only_decorative: 2 own-level awaits, both to non-suspending callees -> A-shallow candidate
{
  const sig = findAndScan("calls_only_decorative")!;
  const body = scanBody(clean, sig.bodyStart!)!;
  check("calls_only_decorative: own await count === 2", body.ownAwaitCount === 2);
  check("calls_only_decorative: callee names resolved", body.awaitCalleeNames.join(",") === "decorative_one,decorative_two");
}

// decorative_one / decorative_two: loop constructs, 0 awaits
{
  const sig1 = findAndScan("decorative_one")!;
  const body1 = scanBody(clean, sig1.bodyStart!)!;
  check("decorative_one: 0 awaits", body1.ownAwaitCount === 0);
  check("decorative_one: has 'for' loop keyword", body1.loopKeywords.has("for"));

  const sig2 = findAndScan("decorative_two")!;
  const body2 = scanBody(clean, sig2.bodyStart!)!;
  check("decorative_two: 0 awaits", body2.ownAwaitCount === 0);
  check("decorative_two: has 'while' loop keyword", body2.loopKeywords.has("while"));
}

// has_nested_fn_helper: nested fn's await must NOT count toward outer
{
  const sig = findAndScan("has_nested_fn_helper")!;
  const body = scanBody(clean, sig.bodyStart!)!;
  check("has_nested_fn_helper: own await count === 0 (nested fn await excluded)", body.ownAwaitCount === 0);
  check("has_nested_fn_helper: has 'for' loop keyword (own level)", body.loopKeywords.has("for"));
}

// closure_await: await inside nested closure/async block DOES count, plus the outer f().await
{
  const sig = findAndScan("closure_await")!;
  const body = scanBody(clean, sig.bodyStart!)!;
  check("closure_await: own await count === 2 (closure-internal + f().await)", body.ownAwaitCount === 2);
}

// raw_string_trap: fake .await / braces inside strings/comments must not confuse the scanner
{
  const sig = findAndScan("raw_string_trap")!;
  check("raw_string_trap: signature has body", sig.hasBody);
  const body = scanBody(clean, sig.bodyStart!)!;
  check("raw_string_trap: parses to a valid close (not null)", body !== null);
  check("raw_string_trap: own await count === 0 (string/comment .await ignored)", body.ownAwaitCount === 0);
  check("raw_string_trap: no loop keywords", body.loopKeywords.size === 0);
}

// declared_only: trait method with no body
{
  const idx = clean.indexOf("fn declared_only");
  const nameEnd = idx + "fn declared_only".length;
  const sig = findBodyOrDecl(clean, nameEnd)!;
  check("declared_only: has no body (trait decl)", sig.hasBody === false);
}

console.log(`\n${pass} passed, ${fail} failed`);
if (fail > 0) process.exit(1);

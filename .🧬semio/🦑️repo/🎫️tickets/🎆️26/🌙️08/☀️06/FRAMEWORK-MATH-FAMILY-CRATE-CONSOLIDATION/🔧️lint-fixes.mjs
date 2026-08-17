#!/usr/bin/env bun
/** 🧹️ Hand-applies the clippy findings that `--fix` could not rewrite automatically. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const MATH = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧮️math";

function edit(rel, subs) {
  const path = join(MATH, rel);
  let text = readFileSync(path, "utf8");
  for (const [from, to] of subs) {
    if (!text.includes(from)) throw new Error(`missing in ${rel}: ${from.slice(0, 60)}`);
    text = text.replace(from, to);
  }
  writeFileSync(path, text);
  console.log(`fixed ${rel} (${subs.length})`);
}

edit("🎲️entropy/🦀️component.rs", [["-1259.139_216_722_402_8", "-1_259.139_216_722_402_8"]]);

edit("🧮️cas/🦀️component.rs", [
  ["fold_radical(b.clone(), e.clone())", "fold_radical(b, e.clone())"],
  ["fn fold_radical(base: Integer, exp: Rational)", "fn fold_radical(base: &Integer, exp: Rational)"],
  ["cramer_solutions(&a, &b, det_a)", "cramer_solutions(&a, &b, &det_a)"],
  ["fn cramer_solutions(a: &[Vec<Expr>], b: &[Expr], det_a: Expr)", "fn cramer_solutions(a: &[Vec<Expr>], b: &[Expr], det_a: &Expr)"],
  [
    "            for row in 0..deg_den {\n                rows[row][col] = basis.coeff(row);\n            }",
    "            for (row, cells) in rows.iter_mut().enumerate().take(deg_den) {\n                cells[col] = basis.coeff(row);\n            }",
  ],
  [
    "    pub struct Rule {\n        lhs: Expr,\n        rhs: RuleRhs,\n        cond: Option<Rc<dyn Fn(&Bindings) -> bool>>,\n    }",
    "    /// 🔍️ Guard evaluated against a candidate match before a rewrite rule fires.\n    pub type RuleCondition = Rc<dyn Fn(&Bindings) -> bool>;\n\n    pub struct Rule {\n        lhs: Expr,\n        rhs: RuleRhs,\n        cond: Option<RuleCondition>,\n    }",
  ],
  ["cond: Rc<dyn Fn(&Bindings) -> bool>) -> Self {", "cond: RuleCondition) -> Self {"],
]);

edit("📈️polynomial/🦀️component.rs", [
  ["        let mut best: Option<(u64, Vec<(PolyU<ModInt>, u32)>)> = None;", "        let mut best: Option<BestFactorization> = None;"],
  ["    enum Combine {", "    #[derive(Clone, Copy)]\n    enum Combine {"],
]);

edit("🕸️graph/🦀️component.rs", [["        assert!(TOL_STRICT < TOL_LOOSE);", "        const { assert!(TOL_STRICT < TOL_LOOSE) };"]]);

edit("🕸️graph/➕️normal/➡️directed/🦀️component.rs", [['assert!(g.get_node_attributes("color").get(&b).is_none());', 'assert!(!g.get_node_attributes("color").contains_key(&b));']]);

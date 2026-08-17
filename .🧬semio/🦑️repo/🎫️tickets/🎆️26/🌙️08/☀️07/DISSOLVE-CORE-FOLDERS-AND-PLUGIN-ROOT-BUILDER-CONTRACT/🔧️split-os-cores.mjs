#!/usr/bin/env bun
import { mkdirSync, readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";

const REPO = "/Users/ueli/Documents/semio";
const OS = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules");

function sliceLines(path, start, end) {
  const lines = readFileSync(path, "utf8").split("\n");
  return lines.slice(start - 1, end).join("\n");
}

function writeComponent(dir, body, doc) {
  const file = join(dir, "🦀️component.rs");
  mkdirSync(dir, { recursive: true });
  const header = doc ? `//! ${doc}\n\n` : "";
  writeFileSync(file, header + body + (body.endsWith("\n") ? "" : "\n"));
}

function splitPack() {
  const src = join(OS, "🎒️pack/🫀️core/🦀️component.rs");
  const doc = readFileSync(src, "utf8").split("\n")[0];
  writeComponent(
    join(OS, "🎒️pack/🆔ids"),
    sliceLines(src, 7, 68),
    "🆔 Pack identity types and segment kind constants.",
  );
  const codecBody = [
    sliceLines(src, 70, 122),
    "",
    "use crate::os_dsl::diagnostic::{FaultOrigin, Severity};",
    "use crate::os_pack::ids::{ByteRange, ChunkId, CodecId, ContentHash, SegmentKind};",
    "",
    sliceLines(src, 124, 477),
  ].join("\n");
  writeComponent(join(OS, "🎒️pack/🧾️codec"), codecBody.replace(
    "crate::os_dsl::core::FaultOrigin::Module",
    "FaultOrigin::Module",
  ).replace(
    "crate::fault_from_thiserror!(PackError, crate::os_dsl::core::FaultOrigin::Module, \"module.pack\");",
    "crate::fault_from_thiserror!(PackError, FaultOrigin::Module, \"module.pack\");",
  ), "🧾 Pack varint, byte I/O, CRC, and compression codec primitives.");

  const sourceBody = [
    "use crate::os_pack::codec::PackError;",
    "",
    sliceLines(src, 366, 477),
  ].join("\n");
  writeComponent(join(OS, "🎒️pack/🚰️source"), sourceBody, "🚰 Random-access PackSource and PackSink traits.");

  const tests = sliceLines(src, 479, 769);
  writeComponent(
    join(OS, "🎒️pack/🧾️codec"),
    readFileSync(join(OS, "🎒️pack/🧾️codec/🦀️component.rs"), "utf8") + "\n" + tests,
    null,
  );
}

function splitDb() {
  const src = join(OS, "🛢️db/🫀️core/🦀️component.rs");
  const idsBody = [
    sliceLines(src, 18, 166),
    "",
    "use pack::PackError;",
  ].join("\n").replaceAll("pack_core::", "pack::");
  writeComponent(join(OS, "🛢️db/🆔ids"), idsBody, "🆔 Db identity types, DbError, and limits.");

  const durBody = sliceLines(src, 168, 369).replaceAll("pack_core::", "pack::");
  writeComponent(
    join(OS, "🛢️db/💾️durability"),
    `use crate::db_ids::{ActorId, DbError, DocumentId};\nuse pack::{ContentHash, PackError};\n\n${durBody}`,
    "💾 Durability class, frontier sync, and epoch fencing.",
  );

  const polBody = sliceLines(src, 371, 519);
  writeComponent(
    join(OS, "🛢️db/🎚️policy"),
    `use crate::db_ids::DbError;\nuse crate::db_durability::DurabilityClass;\n\n${polBody}`,
    "🎚️ Mailbox priority, capabilities, and open profiles.",
  );

  const vgBody = sliceLines(src, 521, 649).replaceAll("pack_core::", "pack::");
  writeComponent(
    join(OS, "🛢️db/🕸️version-graph"),
    `use crate::db_ids::{ActorId, DbError, DocumentId};\nuse pack::ContentHash;\n\n${vgBody}`,
    "🕸️ Version graph seam and Emit observability.",
  );

  const tests = sliceLines(src, 651, 937).replaceAll("pack_core::", "pack::");
  writeComponent(
    join(OS, "🛢️db/🆔ids"),
    readFileSync(join(OS, "🛢️db/🆔ids/🦀️component.rs"), "utf8") + "\n" + tests,
    null,
  );
}

splitPack();
splitDb();
console.log("pack + db splits written");

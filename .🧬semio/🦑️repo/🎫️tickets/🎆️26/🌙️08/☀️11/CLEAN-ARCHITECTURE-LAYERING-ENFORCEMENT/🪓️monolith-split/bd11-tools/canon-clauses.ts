import { join } from "node:path";
import { readFileSync } from "node:fs";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const base = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit";
const parent = readFileSync(join(base, "🦀️.rs"), "utf8") + "\n" + readFileSync(join(base, "../🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs"), "utf8"), borrowed = readFileSync(join(base, "🧵️borrowed/🦀️.rs"), "utf8"), reader = readFileSync(join(base, "📖️reader/🦀️.rs"), "utf8"), native = readFileSync(join(base, "🧳️source/🦀️.rs"), "utf8");
const method = (text: string, pattern: RegExp) => { const start = text.search(pattern); return start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body ?? ""; };
const compact = (text: string) => text.replaceAll(/\s+/g, "");
const indexed = compact(method(parent, /pub fn encode_chunk_admitted\(/)), borrowing = compact(method(borrowed, /fn encode_chunk</)), reading = compact(method(reader, /fn encode_chunk\(/)), sealing = compact(method(parent, /pub fn advance\(/));
const c: Record<string, boolean> = {
  "pub written_bytes": parent.includes("pub written_bytes: usize"), "pub reason": parent.includes("pub reason: ValueError"),
  "indexed progress": indexed.includes("error.retained_progress()"), "indexed reason": indexed.includes("reason:error.with_retained_progress(progress)"), "indexed byte": indexed.includes("ifletSome(byte)=byte{output[0]=byte;copied+=1;}"),
  "borrow1": borrowing.includes("cursor.encode_chunk_admitted(*source,output,child)?"), "borrow2": borrowing.includes("written_bytes:step.written_bytes"),
  "read1": reading.includes("Err(error)=>{self.failed=true;error.written_bytes}"), "read2": reading.includes("self.completed_bytes.checked_add(countasu64)"), "read3": reading.indexOf("self.completed_bytes=completed;") >= 0,
  "seal1": sealing.includes("Err(error)=>(error.retained_progress(),Some(error))"), "seal2": sealing.includes("self.ownership=progress;"), "seal3": sealing.includes("iferror.is_some()||!progress.fits(retained){self.cancelled=true;}"), "seal4": sealing.indexOf("self.completed_bytes=completed;") >= 0, "seal5": sealing.includes("!grant.permits_one()||self.cancelled||self.closing"),
  "native": compact(native).includes('refusal("canonical original child receipt exceeds caller grant").with_retained_progress(p)'.replaceAll(/\s+/g, "")),
};
for (const [k, v] of Object.entries(c)) if (!v) console.log("FALSE", k);
console.log("lens", indexed.length, borrowing.length, reading.length, sealing.length);
console.log("SEAL:", sealing.slice(0, 700));

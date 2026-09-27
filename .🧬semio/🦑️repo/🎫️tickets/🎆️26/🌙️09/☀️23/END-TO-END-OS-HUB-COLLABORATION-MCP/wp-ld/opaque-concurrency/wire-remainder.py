#!/usr/bin/env python3
"""✉️ LD item 2, remainder of the envelope wire change (observed + target), applied in ONE step:
the app-channel paged envelope writer, the sync actor's message size accounting, the hub's canonical
inference command (a protocol envelope record: server-stamped approvals carry no observation and the
whole-artifact target) with its two TS oracles, and every hub fixture pinning a command's bytes/hash.

usage: wire-remainder.py [--dry-run]
"""
import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
CHANNEL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs"
SYNC = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"
COMMAND = ROOT / "🌎️hub/💡️inference/✉️command/🦀️.rs"
SCRIPT = ROOT / "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
BATCH = ROOT / "🧰️framework/🔨️modules/📡️replication/🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"
FIXTURES = [ROOT / "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json", ROOT / "🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json"]

EDITS = [
    (CHANNEL, """        for dependency in &value.dependencies {
            self.string(&dependency.0)?;
        }
        self.string(&value.diff.schema.0)?;""", """        for dependency in &value.dependencies {
            self.string(&dependency.0)?;
        }
        match &value.observed {
            Some(observed) => {
                self.varint(1)?;
                self.string(&observed.0)?;
            }
            None => self.varint(0)?,
        }
        self.varint(value.target.len() as u64)?;
        for segment in &value.target {
            self.string(segment)?;
        }
        self.string(&value.diff.schema.0)?;"""),
    (SYNC, """                for dependency in &envelope.dependencies {
                    text(&mut bytes, &dependency.0)?;
                }
                text(&mut bytes, &envelope.diff.schema.0)?;""", """                for dependency in &envelope.dependencies {
                    text(&mut bytes, &dependency.0)?;
                }
                add(&mut bytes, 1)?;
                if let Some(observed) = &envelope.observed {
                    text(&mut bytes, &observed.0)?;
                }
                add(&mut bytes, 4)?;
                for segment in &envelope.target {
                    text(&mut bytes, segment)?;
                }
                text(&mut bytes, &envelope.diff.schema.0)?;"""),
    (COMMAND, """            dependencies[index] = value;
        }
        let value = Self {""", """            dependencies[index] = value;
        }
        if cursor.integer()? != 0 || cursor.integer()? != 0 {
            return Err(InferenceErrorV1::Invalid);
        }
        let value = Self {"""),
    (COMMAND, """        for dependency in &self.dependencies[..self.dependency_count] {
            protocol::write_str(output, dependency);
        }
        protocol::write_str(output, self.diff_schema);""", """        for dependency in &self.dependencies[..self.dependency_count] {
            protocol::write_str(output, dependency);
        }
        protocol::wire::write_varint_u64(output, 0);
        protocol::wire::write_varint_u64(output, 0);
        protocol::write_str(output, self.diff_schema);"""),
    (SCRIPT, """  integer(command.dependencies.length);
  command.dependencies.forEach(text);
  text(command.diff.schema);""", """  integer(command.dependencies.length);
  command.dependencies.forEach(text);
  integer(0);
  integer(0);
  text(command.diff.schema);"""),
    (SCRIPT, """    parts.push(variable(command.dependencies.length));
    command.dependencies.forEach(text);
    text(command.diff.schema);""", """    parts.push(variable(command.dependencies.length));
    command.dependencies.forEach(text);
    parts.push(variable(0), variable(0));
    text(command.diff.schema);"""),
    (SCRIPT, """      command.dependencies.push(value);
    }
    command.diff = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };""", """      command.dependencies.push(value);
    }
    if (integer() !== 0 || integer() !== 0) throw new Error("server-stamped command carries an observation or a target");
    command.diff = { schema: text(), payloadHex: field(limits.payloadBytes).toString("hex") };"""),
]


def varint_end(raw, position):
    while raw[position] & 0x80:
        position += 1
    return position + 1


def read_varint(raw, position):
    value, shift = 0, 0
    while True:
        byte = raw[position]
        position += 1
        value |= (byte & 0x7F) << shift
        shift += 7
        if not byte & 0x80:
            return value, position


def with_empty_observation(raw):
    position = 0
    for _ in range(3):
        length, position = read_varint(raw, position)
        position += length
    count, position = read_varint(raw, position)
    for _ in range(count):
        length, position = read_varint(raw, position)
        position += length
    return raw[:position] + b"\x00\x00" + raw[position:]


def main() -> int:
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS} | set(FIXTURES) | {BATCH}}
    failures = 0
    for index, (path, before, after) in enumerate(EDITS):
        count = texts[path].count(before)
        if count != 1:
            print(f"edit {index} {path.name}: anchor found {count} times", file=sys.stderr)
            failures += 1
            continue
        texts[path] = texts[path].replace(before, after, 1)
    hashes = {}
    for path in FIXTURES:
        text = texts[path]
        for match in re.finditer(r'"(encodedHex|commandHex)": "([0-9a-f]+)"', text):
            old = bytes.fromhex(match.group(2))
            new = with_empty_observation(old)
            hashes[hashlib.sha256(old).hexdigest()] = hashlib.sha256(new).hexdigest()
            text = text.replace(match.group(2), new.hex())
        texts[path] = text
    for path in FIXTURES:
        for old, new in hashes.items():
            texts[path] = texts[path].replace(old, new)
    print("command hashes:", hashes)
    batch = texts[BATCH]
    before = '"expect": { "outcome": "accepted", "reason": "canonical", "envelopes": [{ "mutationId": "m", "documentId": "d", "actor": "a", "dependencies": [], "observed": null, "target": [], "diff": { "schema": "s", "payloadHex": "aa" }, "inverse": { "schema": "i", "payloadHex": "bbcc" }, "timestamp": { "actor": "3", "physicalMs": "4", "logical": "5" } }] }\n    },\n    {\n      "id": "observed-flag-invalid"'
    after = before.replace('"observed": null, "target": []', '"observed": "o", "target": ["tiles", "t-hero"]')
    if batch.count(before) != 1:
        print(f"batch fixture anchor found {batch.count(before)} times", file=sys.stderr)
        failures += 1
    else:
        texts[BATCH] = batch.replace(before, after, 1)
    if failures:
        return 1
    for path in FIXTURES + [BATCH]:
        json.loads(texts[path])
    if not dry:
        for path, text in texts.items():
            path.write_text(text, encoding="utf-8")
    print(f"{len(EDITS)} edits + {len(FIXTURES)} hub fixtures + batch fixture ok{' (dry run)' if dry else ''}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

/** 🧪️ Independent reference for the fixture's limited authored wire shapes; no Rust implementation imports. */
const fs = require("node:fs");
const assert = require("node:assert/strict");
const fixturePath = process.argv[2];
const fixture = JSON.parse(fs.readFileSync(fixturePath, "utf8"));
function unsigned(value) {
    assert(Number.isSafeInteger(value) && value >= 0);
    const bytes = [];
    do { const byte = value % 128; value = Math.floor(value / 128); bytes.push(byte | (value ? 128 : 0)); } while (value);
    return bytes;
}
function utf8(text) { return [...unsigned(Buffer.byteLength(text)), ...Buffer.from(text)]; }
function frame(row) {
    const { mutation, ...payload } = row.value;
    const strings = [];
    function scan(value) {
        if (typeof value === "string") strings.push(value);
        else if (Array.isArray(value)) value.forEach(scan);
        else if (value && typeof value === "object") Object.values(value).forEach(scan);
    }
    scan(payload);
    assert(strings.every(text => Buffer.byteLength(text) <= 128));
    const symbols = [...new Set(strings)].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
    function value(item) {
        if (item === null) return [18];
        if (typeof item === "string") return [6, ...unsigned(symbols.indexOf(item))];
        if (typeof item === "number") return [4, ...unsigned(item)];
        if (Array.isArray(item)) return [12, ...unsigned(item.length), ...item.flatMap(value)];
        return [16, ...unsigned(Object.keys(item).length), ...Object.entries(item).flatMap(([key, child]) => [7, ...utf8(key), ...value(child)])];
    }
    return Buffer.from([1, row.tag, ...unsigned(symbols.length), ...symbols.flatMap(utf8), 1, 1, 17, ...value(payload)]);
}
for (const row of fixture.cases) assert.equal(frame(row).toString("hex"), row.binaryHex);
process.stdout.write(JSON.stringify({ cases: fixture.cases.length, verbs: new Set(fixture.cases.map(row => row.tag)).size }) + "\n");
module.exports = { frame };

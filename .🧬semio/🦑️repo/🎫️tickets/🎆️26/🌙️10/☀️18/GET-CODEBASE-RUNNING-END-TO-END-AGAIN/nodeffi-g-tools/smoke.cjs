const a = require(process.argv[2]);
console.log(Object.keys(a));
const h = a.open("/usr/lib/libSystem.B.dylib");
const getpid = a.symbol(h, "getpid");
console.log(Number(a.call(getpid) & 0xffffffffn), process.pid);
const err = a.symbol(h, "__error");
const p = a.call(err);
console.log(p > 0n, new DataView(a.peek(p, 4n)).getInt32(0, true));
const buf = Buffer.from("hello\0");
const strlen = a.symbol(h, "strlen");
console.log(a.call(strlen, a.addressOf(buf)), a.addressOf(buf).toString(16));
const v = a.view(a.addressOf(buf), 5n); console.log(Buffer.from(v).toString());
const u = new Uint8Array(buf.buffer, buf.byteOffset+1, 3); console.log(a.addressOf(u) - a.addressOf(buf));
try { a.symbol(h, "nope_nope"); } catch (e) { console.log("err", e.message); }
try { a.open("/nonexistent.dylib"); } catch (e) { console.log("err", e.message.slice(0,60)); }
console.log(a.close(h));

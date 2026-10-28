import { dlopen, FFIType as f, read, ptr, toArrayBuffer } from "bun:ffi";
const open = (symbols: any) => dlopen("/usr/lib/libSystem.B.dylib", symbols).symbols;
const show = (label: string, v: unknown) => console.log(label.padEnd(34), typeof v, v === null ? "null" : String(v));
// same symbol bound under different return types needs separate dlopen calls
for (const t of ["i8","u8","i16","u16","i32","u32","i64","u64","bool","ptr","void"]) {
  const x = open({ getpid: { args: [], returns: t } }); show("getpid returns " + t, x.getpid());
}
for (const t of ["i64","u64","i32","u32"]) { const x = open({ labs: { args: [t], returns: t } }); try { show("labs("+t+") of -5", x.labs(-5)); } catch (e) { show("labs("+t+") -5 throws", (e as Error).message); } try { show("labs("+t+") of -5n", x.labs(-5n)); } catch (e) { show("labs("+t+") -5n throws", (e as Error).message); } }
const x = open({ abs: { args: ["i32"], returns: "i32" }, strlen: { args: ["ptr"], returns: "u64" } });
show("abs(-7)", x.abs(-7));
show("strlen(Buffer)", x.strlen(Buffer.from("abc\0")));
show("strlen(Uint8Array)", x.strlen(new Uint8Array([97,98,0])));
show("strlen(number ptr)", x.strlen(ptr(Buffer.from("abcd\0"))));
const y = open({ strlen: { args: ["cstring"], returns: "u64" } });
show("strlen cstring Buffer", y.strlen(Buffer.from("abcde\0")));
try { show("strlen cstring string", y.strlen("xyz")); } catch (e) { show("cstring string throws", (e as Error).message); }
const z = open({ toupper: { args: ["i32"], returns: "i32" } });
show("toupper(97)", z.toupper(97));
show("toupper(true)", (()=>{ try { return z.toupper(true as any); } catch (e) { return "throws " + (e as Error).message; } })());
show("ptr(Buffer) typeof", ptr(Buffer.alloc(8)));
const b = Buffer.from("xyz12345"); const sub = b.subarray(3);
show("ptr(sub) - ptr(b)", ptr(sub) - ptr(b));
const e = open({ __error: { args: [], returns: "ptr" } });
const p = e.__error(); show("errno ptr", p); show("read.i32", read.i32(p)); show("read.i64", read.i64(p)); show("read.u8", read.u8(p)); show("read.ptr", read.ptr(p));
show("read.i32(p,0)", read.i32(p, 0));
show("toArrayBuffer len", toArrayBuffer(p, 0, 4).byteLength);
try { open({ nope_nope: { args: [], returns: "i32" } }); } catch (er) { show("missing symbol throws", (er as Error).message); }
try { dlopen("/nonexistent.dylib", { a: { args: [], returns: "i32" } }); } catch (er) { show("missing lib throws", (er as Error).message.slice(0, 80)); }
const q = open({ qsort: { args: ["ptr","u64","u64","ptr"], returns: "void" } });
show("void return", q.qsort(Buffer.alloc(0), 0, 1, null));
const u = open({ strtoull: { args: ["ptr","ptr","i32"], returns: "u64" } });
show("strtoull max", u.strtoull(Buffer.from("18446744073709551615\0"), null, 10));
const v = open({ strtoll: { args: ["ptr","ptr","i32"], returns: "i64" } });
show("strtoll neg", v.strtoll(Buffer.from("-9223372036854775807\0"), null, 10));
show("strtoll small", v.strtoll(Buffer.from("-5\0"), null, 10));
const w = open({ strtoull: { args: ["ptr","ptr","i32"], returns: "u64" } });
show("strtoull small", w.strtoull(Buffer.from("5\0"), null, 10));
const g = open({ getenv: { args: ["cstring"], returns: "cstring" } });
const r: any = g.getenv(Buffer.from("HOME\0")); show("cstring return", r); console.log(Object.prototype.toString.call(r), r && r.constructor && r.constructor.name);
const g2 = open({ getenv: { args: ["cstring"], returns: "ptr" } });
show("getenv ptr", g2.getenv(Buffer.from("NOPE_NOT_SET\0")));

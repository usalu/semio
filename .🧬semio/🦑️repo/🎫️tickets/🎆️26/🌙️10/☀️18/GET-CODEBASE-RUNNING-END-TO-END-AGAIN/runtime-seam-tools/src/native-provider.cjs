const symbolsOf = {
  add: (left, right) => left + right,
  getpid: () => process.pid,
};
exports.nativeSystemV1 = {
  FFIType: { void: "void", bool: "bool", i8: "i8", u8: "u8", i16: "i16", u16: "u16", i32: "i32", u32: "u32", i64: "i64", u64: "u64", f32: "f32", f64: "f64", ptr: "ptr", cstring: "cstring" },
  dlopen: (path, declared) => ({ symbols: Object.fromEntries(Object.keys(declared).map(name => [name, symbolsOf[name]])), close() {}, path }),
  read: { i32: () => 7 },
  ptr: () => 1,
  toArrayBuffer: () => new ArrayBuffer(4),
  JSCallback: class { constructor(body) { this.pointer = 9; this.body = body; } close() {} },
};

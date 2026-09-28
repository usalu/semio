/** 🔎️ C12: the payload projection of the prepared child-payload patch, run on the law's own input. */
const text = (read: () => unknown): string | null => { try { const value = read(); return typeof value === "string" ? value : null; } catch { return null; } };
const error = Object.assign(new Error("[object Object] (see error.payload)"), { name: "ComponentError", payload: { tag: "fault", val: { code: "duplicate mutation id", at: 7n, bytes: new Uint8Array(3) } } });
const payload = text(() => { const value = (error as { payload?: unknown })?.payload; return value === undefined ? undefined : JSON.stringify(value, (_key, item: unknown) => (typeof item === "bigint" ? item.toString() : item instanceof Uint8Array ? `<${item.byteLength} bytes>` : item)); });
console.log(payload ?? text(() => error.message));
const plain = new TypeError("guest refused the activation turn");
console.log(text(() => { const value = (plain as { payload?: unknown })?.payload; return value === undefined ? undefined : JSON.stringify(value); }) ?? plain.message);

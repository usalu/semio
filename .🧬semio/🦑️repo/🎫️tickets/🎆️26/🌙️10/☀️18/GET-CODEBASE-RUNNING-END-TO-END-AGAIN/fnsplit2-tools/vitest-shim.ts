export const trace: string[] = [];
const depth: string[] = [];
const normalize = (source: string) => source.split("\n").map((l) => l.trim()).join("\n").replace(/(\.\.\/)+/g, "../");
const hashText = (s: string) => { let h = 0xcbf29ce484222325n; for (const ch of s) { h ^= BigInt(ch.codePointAt(0)!); h = (h * 0x100000001b3n) & 0xffffffffffffffffn; } return h.toString(16).slice(0, 16); };
const stub = (): any => new Proxy(function () {}, { get: (_t, k) => (k === Symbol.toPrimitive ? () => 1 : k === "then" ? undefined : stub()), apply: () => stub(), construct: () => stub() as object });
const recorder = (kind: string): any => {
  const call = (...a: any[]) => {
    const name = typeof a[0] === "string" ? a[0] : "";
    const fn = a.find((x) => typeof x === "function");
    if (typeof a[0] !== "string" && !fn) return recorder(`${kind}()`);
    const nests = kind.startsWith("describe") || kind === "suite";
    trace.push(`${depth.join(" > ")} :: ${kind}${nests ? "+" : ""} ${name} #${fn && !nests ? hashText(normalize(String(fn))) : "none"}`);
    if (nests && fn) { depth.push(name); const r = fn(); depth.pop(); trace.push(`${depth.join(" > ")} :: ${kind}- ${name}`); return r; }
    return undefined;
  };
  return new Proxy(call, { get: (t, k) => (typeof k === "string" && k !== "then" ? recorder(`${kind}.${k}`) : (t as any)[k]), apply: (_t, _this, a) => call(...a) });
};
export const describe = recorder("describe"), it = recorder("it"), test = recorder("test"), beforeEach = recorder("beforeEach"), afterEach = recorder("afterEach"), beforeAll = recorder("beforeAll"), afterAll = recorder("afterAll");
export const expect = stub(), vi = stub(), suite = recorder("suite");

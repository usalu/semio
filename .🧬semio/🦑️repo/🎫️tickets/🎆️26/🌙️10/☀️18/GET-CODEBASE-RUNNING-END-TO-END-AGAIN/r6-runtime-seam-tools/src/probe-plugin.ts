import { runtimeLoaderPluginV1 } from "../../🟦️.ts";

runtimeLoaderPluginV1({
  name: "probe",
  setup(build) {
    build.onResolve({ filter: /entry\.virtual\.mjs$/u }, () => ({ path: "/virtual/main", namespace: "probe" }));
    build.onResolve({ filter: /^\.\/virtual-/u }, arguments_ => ({ path: `/virtual/${arguments_.path.slice("./virtual-".length).replace(/\.mjs$/u, "")}`, namespace: "probe" }));
    build.onLoad({ filter: /^\/virtual\/main$/u, namespace: "probe" }, () => ({ contents: "import hello, { name } from './virtual-hello.mjs'; import object, { answer, call } from './virtual-object.mjs'; export { hello, name, answer, call, object };", loader: "js" }));
    build.onLoad({ filter: /^\/virtual\/hello$/u, namespace: "probe" }, () => ({ contents: "export default 42; export const name = 'hello';", loader: "js" }));
    build.onLoad({ filter: /^\/virtual\/object$/u, namespace: "probe" }, () => ({ loader: "object", exports: { answer: 7, call: () => "called", default: { answer: 7 } } }));
  },
});
const main = await import(new URL("./entry.virtual.mjs", import.meta.url).pathname);
console.log(JSON.stringify({ hello: main.hello, name: main.name, answer: main.answer, call: main.call(), object: main.object }));

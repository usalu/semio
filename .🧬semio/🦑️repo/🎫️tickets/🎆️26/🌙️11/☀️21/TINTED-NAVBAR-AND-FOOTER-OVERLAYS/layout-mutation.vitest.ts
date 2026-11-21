import type { Plugin } from "vite";
import config from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const mutations = {
  sticky: '[data-slot="layout-scroll-start"],[data-slot="layout-scroll-end"]{position:static!important}',
  tint: ".ui-chrome-tint{background-color:transparent!important}",
};
const selected = process.env.SEMIO_LAYOUT_TEST_MUTATION as keyof typeof mutations;
if (!Object.hasOwn(mutations, selected)) throw new Error("Select the sticky or tint mutation");

const mutate: Plugin = {
  name: "layout-overlay-test-mutation",
  enforce: "pre",
  transform(source, id) {
    if (!id.endsWith("/📐️Layout/🧪️tests/🧩️component/🟦️.tsx")) return;
    const marker = 'const css = compiler.build(classes)';
    if (!source.includes(marker)) throw new Error("Layout stylesheet fixture has moved");
    return source.replace(marker, marker + " + " + JSON.stringify(mutations[selected]));
  },
};

export default { ...config, plugins: [mutate] };

import base from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const transient = "C:/git/semio/🧰️framework/🔨️modules/🫧️transient/🟦️.ts";

export default {
  ...base,
  resolve: { ...base.resolve, alias: [...(base.resolve!.alias as { find: RegExp; replacement: string }[]), { find: /^@semio-tech\/framework-transient$/u, replacement: transient }] },
};

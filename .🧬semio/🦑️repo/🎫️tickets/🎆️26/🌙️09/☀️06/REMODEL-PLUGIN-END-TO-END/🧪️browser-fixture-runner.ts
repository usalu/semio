import * as testModule from "bun:test";

Bun.plugin({
  name: "remodel-browser-fixture-runner",
  setup(builder) {
    builder.module("vitest", () => ({ exports: { ...testModule }, loader: "object" }));
  },
});

import base from "../q2-vitest-alias.config.ts";

export default {
  ...base,
  root: import.meta.dirname,
  server: { fs: { strict: false } },
  test: { ...base.test, root: import.meta.dirname, include: ["probe.test.ts"], exclude: ["**/node_modules/**"] },
};

import { describe, expect, it } from "vitest";
import { createUiReactLintConfig } from "../../🟦️lint-tooling.ts";

describe("owned UI React lint config", () => {
  it("does not depend on predefined globals or enable no-undef", () => {
    const config = createUiReactLintConfig();
    expect(config.every((entry) => !("globals" in ((entry.languageOptions as Record<string, unknown> | undefined) ?? {})))).toBe(true);
    expect(config.every((entry) => !("no-undef" in ((entry.rules as Record<string, unknown> | undefined) ?? {})))).toBe(true);
  });
});

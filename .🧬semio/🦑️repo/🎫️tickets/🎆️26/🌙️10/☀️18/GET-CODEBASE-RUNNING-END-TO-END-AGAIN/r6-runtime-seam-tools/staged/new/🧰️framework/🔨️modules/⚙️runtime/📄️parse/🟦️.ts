import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";

/** 📄️ Parses TOML text. */
export function runtimeParseTomlV1(text: string): any { return requireRuntimeMemberV1("toml")(text); }
/** 📝️ Parses JSON with comments. */
export function runtimeParseJsoncV1(text: string): any { return requireRuntimeMemberV1("jsonc")(text); }
/** 📃️ Parses YAML text. */
export function runtimeParseYamlV1(text: string): any { return requireRuntimeMemberV1("yaml")(text); }

import type { JsonSyntaxNode } from "../../../🎒️pack/🔤️json/📥️decode/🟦️.ts";

/** 🌈️ Projects owned integer channel words into the canonical sRGB8888 color input. */
export function rgba8FromOwnedValues(values: readonly JsonSyntaxNode[]): readonly [number, number, number, number] | null {
  if (values.length < 3) return null;
  const channel = (value: JsonSyntaxNode | undefined, fallback: number): number => {
    if (value?.kind !== "number" || !/^(0|[1-9][0-9]*)$/u.test(value.text)) return fallback;
    const integer = BigInt(value.text);
    if (integer > 18446744073709551615n) return fallback;
    return Number(integer > 255n ? 255n : integer);
  };
  return [channel(values[0], 0), channel(values[1], 0), channel(values[2], 0), channel(values[3], 255)];
}

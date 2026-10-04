/** 🗣️ Compares the spoken remaining time of the client with `Intl.DurationFormat`'s long style and probes typed SI prefixes. Usage: `bun duration_words_probe.ts`. */
import { floorSignificant, formatDuration, formatFactor, parseQuantity } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/📏️quantity/🟦️.ts";

const DurationFormat = (Intl as unknown as { readonly DurationFormat: new (locale: string, options: object) => { format(duration: object): string } }).DurationFormat;
for (const locale of ["en", "de"] as const) {
  for (const left of [0, 1, 1000, 5_000, 48_000, 60_000, 61_000, 125_000, 754_000]) {
    const seconds = Math.ceil(left / 1000);
    const oracle = new DurationFormat(locale, { style: "long" }).format({ minutes: Math.floor(seconds / 60), seconds: seconds % 60 });
    console.log(`[DEBUG] ${locale} ${left} ours=${JSON.stringify(formatDuration(left, locale))} intl=${JSON.stringify(oracle)}`);
  }
}
for (const [text, unit] of [["30 cm", "m"], ["100 hPa", "Pa"], ["5 M", "m"], ["5 M", "W"], ["5 dam", "m"], ["5 G", "g"], ["5 m", "m"]] as const) console.log(`[DEBUG] ${text} ${unit} ${parseQuantity(text, { unit, prefixed: true, scale: "logarithmic" }, "en")}`);
for (const factor of [1.96, 1.04, 2.3, 1000, 160.3, 9.99]) console.log(`[DEBUG] ${factor} ${floorSignificant(factor, 2)} ${formatFactor(factor, "en")}`);

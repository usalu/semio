
/** 🙅️ The frozen fault codes of a refused example switch — `ExampleRefusalCodeV1` (`🧬️schema/🔣️.json`), twin of Rust
 * `ExampleRefusalKind::code`; both are pinned by `🧫️fixtures/📚️example-refusal.json`. */
export const EXAMPLE_REFUSAL_CODES_V1 = ["example.unknown", "example.undecodable", "example.empty"] as const;
export type ExampleRefusalCodeV1 = (typeof EXAMPLE_REFUSAL_CODES_V1)[number];

/** 🗣️ The notice a shell shows per example refusal, in English and German — `ExampleRefusalMessagesV1`, twin of Rust
 * `ExampleRefusalKind::label`. */
export const EXAMPLE_REFUSAL_MESSAGES_V1: Readonly<Record<ExampleRefusalCodeV1, { readonly en: string; readonly de: string }>> = {
  "example.unknown": { en: "This app has no example with that name.", de: "Diese App hat kein Beispiel mit diesem Namen." },
  "example.undecodable": { en: "The example could not be read: its file is outdated or damaged.", de: "Das Beispiel konnte nicht gelesen werden: Seine Datei ist veraltet oder beschädigt." },
  "example.empty": { en: "The example file is empty, so nothing was loaded.", de: "Die Beispieldatei ist leer, daher wurde nichts geladen." },
};

/** 🔎️ `true` for a fault code that names an example refusal. */
export function isExampleRefusalCodeV1(code: string): code is ExampleRefusalCodeV1 {
  return (EXAMPLE_REFUSAL_CODES_V1 as readonly string[]).includes(code);
}

/** 🗣️ The notice for one example refusal in `locale`, chosen where the refusal is decided (the input-ledger rule):
 * `de` reads German, every other locale English. */
export function exampleRefusalNoticeTextV1(code: ExampleRefusalCodeV1, locale: string): string {
  const message = EXAMPLE_REFUSAL_MESSAGES_V1[code];
  return locale === "de" ? message.de : message.en;
}

/** 🧮️ A declared example's decode outcome, as the example-seat predicate reads it. */
export type ExampleDecodeV1 = "document" | "genesis" | "undecodable";

/** 🎯️ What one example switch seats, or the refusal it answers instead. */
export type ExampleSeatOutcomeV1 = { readonly seat: "genesis" | "example" } | { readonly refusal: ExampleRefusalCodeV1 };

/** 🎯️ THE example-seat predicate — twin of Rust `example_seat` + `example_decoded`: the empty id seats the app's own
 * genesis document, an undeclared id is refused before anything decodes, and a declared example seats unless its asset
 * does not decode or decodes into the genesis document. */
export function exampleSeatOutcomeV1(exampleId: string, declared: readonly string[], decode: ExampleDecodeV1): ExampleSeatOutcomeV1 {
  if (exampleId === "") return { seat: "genesis" };
  if (!declared.includes(exampleId)) return { refusal: "example.unknown" };
  if (decode === "undecodable") return { refusal: "example.undecodable" };
  if (decode === "genesis") return { refusal: "example.empty" };
  return { seat: "example" };
}

const protocol = read("../../../../../../🔨️modules/⚙️runtime/🗣️language/🔣️.json");
const shared = read("../../../../../../🔨️modules/⚙️runtime/🗣️language/🧪️tests/🔣️.json");

const resolve = (environment: Record<string, string>): string | null => {
  const explicit = environment[protocol.explicitVariable];
  if (explicit) return intlLanguage(explicit.trim());
  for (const entry of (environment[protocol.preferenceListVariable] ?? "").split(protocol.preferenceListSeparator)) { const named = entry ? intlLanguage(entry) : null; if (named) return named; }
  const name = protocol.environmentOrder.find((candidate: string) => environment[candidate]);
  return name === undefined ? null : intlLanguage(environment[name]!);
};

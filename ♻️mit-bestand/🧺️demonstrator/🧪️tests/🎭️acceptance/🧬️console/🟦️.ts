/** 🔎️ Preserves every console error except the browser’s exact missing-resource diagnostic. */
export function significantConsoleErrors(messages: readonly string[]): string[] {
  return messages.filter((message) => !/^Failed to load resource: the server responded with a status of 404(?: \(Not Found\))?$/i.test(message));
}

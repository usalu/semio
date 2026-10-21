/** 🆕️ Requires a real distinct group before targeting a restarted tool run. */
export function isNewToolRunGroup(previous: string, candidate: string): boolean {
  return /^framework\.toolRun\.[0-9]+$/.test(candidate) && candidate !== previous;
}

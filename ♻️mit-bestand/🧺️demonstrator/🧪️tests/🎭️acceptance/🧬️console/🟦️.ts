/** 🔎️ Preserves every console error except the browser’s exact missing-resource diagnostic. */
export function significantConsoleErrors(messages: readonly string[]): string[] {
  return messages.filter((message) => !/^Failed to load resource: the server responded with a status of 404(?: \(Not Found\))?$/i.test(message));
}

/** 🌐️ Detect requests for machine-local services when exercising a release on a static host. */
export function isLocalReleaseDependency(value: string, site: URL): boolean {
  const url = new URL(value);
  if (url.protocol === "file:") return true;
  if (!["http:", "https:", "ws:", "wss:"].includes(url.protocol)) return false;
  if (url.host === site.host) return url.pathname === "/__semio/agent-bridge";
  return /^(?:127\.|localhost$|.*\.localhost$|\[::1\]$)/i.test(url.hostname.replace(/\.$/, ""));
}

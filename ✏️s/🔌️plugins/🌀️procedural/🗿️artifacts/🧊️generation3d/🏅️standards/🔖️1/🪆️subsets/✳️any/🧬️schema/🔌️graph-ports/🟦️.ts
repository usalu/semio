/** 🧷️ The authored widget, channel and direction of a graph port. */
export interface GraphPortAddress {
  widget: string;
  channel: string;
  input: boolean;
}

/** 🎯️ Resolves one canonical graph handle to its authored port. */
export function parseGraphPortId(id: string): GraphPortAddress | undefined {
  const separator = id.lastIndexOf("@");
  if (separator < 1) return undefined;
  let code = id.slice(separator + 1);
  const input = code.startsWith("!i");
  if (input) {
    code = code.slice(2);
  } else if (code.startsWith("!o")) {
    code = code.slice(2);
    if (code && !code.startsWith("!")) return undefined;
  } else if (!code || code.startsWith("!")) {
    return undefined;
  }
  if (/[#~]|%(?!25|40|23|7E)/u.test(code)) return undefined;
  return { widget: id.slice(0, separator), channel: decodeURIComponent(code), input };
}

/** 📍️ Resolves a viewport instance to its authored port and index. */
export function parseGraphInstanceId(id: string): { port: GraphPortAddress; index: number } | undefined {
  const separator = id.lastIndexOf("#");
  if (separator < 0) return undefined;
  const port = parseGraphPortId(id.slice(0, separator));
  if (!port) return undefined;
  let text = id.slice(separator + 1);
  if (port.input) {
    if (!text.startsWith("in-")) return undefined;
    text = text.slice(3);
  }
  if (!/^(?:0|[1-9][0-9]*)(?![\s\S])/u.test(text)) return undefined;
  const index = Number(text);
  return Number.isSafeInteger(index) ? { port, index } : undefined;
}

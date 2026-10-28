/** 🧪️ Test-only XMLBuilder/XMLParser boundary for owned semantic XML nodes. */
export function createTextXmlOracle(library) {
  const options = { preserveOrder: true, ignoreAttributes: false, parseTagValue: false, parseAttributeValue: false, trimValues: false, processEntities: true, commentPropName: "#comment", cdataPropName: "#cdata", ignoreDeclaration: true };
  const parser = new library.XMLParser(options);
  const builder = new library.XMLBuilder({ ...options, format: false, suppressEmptyNode: true });
  const fromRow = (row) => {
    const name = Object.keys(row).find((key) => key !== ":@");
    if (name === "#text") return { kind: "text", text: String(row[name]) };
    if (name === "#cdata" || name === "#comment") return { kind: name === "#cdata" ? "cData" : "comment", text: row[name].map((part) => part["#text"] ?? "").join("") };
    if (name?.startsWith("?")) return { kind: "processingInstruction", target: name.slice(1), data: row[name].map((part) => part["#text"] ?? "").join("") };
    if (!name) throw new Error("oracle: XML node has no name");
    return { kind: "element", name, attrs: Object.entries(row[":@"] ?? {}).map(([key, value]) => ({ name: key.slice(2), value: String(value) })), children: row[name].map(fromRow) };
  };
  const toRow = (node) => {
    if (node.kind === "text") return { "#text": node.text };
    if (node.kind === "cData" || node.kind === "comment") return { [node.kind === "cData" ? "#cdata" : "#comment"]: [{ "#text": node.text }] };
    if (node.kind === "processingInstruction") return { ["?" + node.target]: [{ "#text": node.data }] };
    return { [node.name]: node.children.map(toRow), ":@": Object.fromEntries(node.attrs.map((attribute) => ["@_" + attribute.name, attribute.value])) };
  };
  return {
    read(source) {
      if (library.XMLValidator.validate(source) !== true) throw new Error("oracle: XML is not well formed");
      const root = parser.parse(source).map(fromRow).find((node) => node.kind === "element");
      if (!root) throw new Error("oracle: XML root missing");
      return root;
    },
    write(node) { return builder.build([toRow(node)]); },
  };
}

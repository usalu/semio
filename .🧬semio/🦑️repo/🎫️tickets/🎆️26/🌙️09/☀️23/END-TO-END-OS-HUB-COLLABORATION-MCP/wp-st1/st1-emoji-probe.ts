/** 🔎️ ST1 probe: taxonomy emoji admission (RGI or Extended_Pictographic+FE0F), as `canonicalTaxonomyEmoji` rules. */
const RGI = new RegExp("^(?:\\p{RGI_Emoji})$", "v");
for (const name of process.argv.slice(2)) {
  const emoji = [...new Intl.Segmenter().segment(name)][0]!.segment;
  const ok = emoji === emoji.normalize("NFC") && (RGI.test(emoji) || /^\p{Extended_Pictographic}️$/u.test(emoji));
  console.log(name, JSON.stringify([...emoji].map((c) => c.codePointAt(0)!.toString(16))), ok ? "admitted" : "REFUSED");
}

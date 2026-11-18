export default (a) => {
  const s50 = a.volumes["50"].body, s100 = a.volumes["100"].body;
  const ids = (process.env.CP ?? "chain-top,chain-bottom,rear-cog,chainring").split(",");
  const at = s100.findIndex((s) => s.id === "bottom-bracket");
  s100.splice(at + 1, 0, ...ids.filter((id) => !s100.some((s) => s.id === id)).map((id) => JSON.parse(JSON.stringify(s50.find((s) => s.id === id)))));
};

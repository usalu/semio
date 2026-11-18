export default (a) => {
  const src = a.volumes["100"].body.find((s) => s.id === "head-cap-brim");
  const b = a.volumes["200"].body;
  if (!b.some((s) => s.id === "head-cap-brim")) b.splice(b.findIndex((s) => s.id === "head-cap") + 1, 0, JSON.parse(JSON.stringify(src)));
};

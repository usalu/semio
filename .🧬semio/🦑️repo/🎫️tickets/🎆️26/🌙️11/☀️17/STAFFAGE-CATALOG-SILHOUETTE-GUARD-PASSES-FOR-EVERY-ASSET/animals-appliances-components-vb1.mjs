export default (a) => {
  for (const sc of [100, 200]) for (const s of a.volumes[sc].body) if (s.id.startsWith("guide")) delete s.slender;
};

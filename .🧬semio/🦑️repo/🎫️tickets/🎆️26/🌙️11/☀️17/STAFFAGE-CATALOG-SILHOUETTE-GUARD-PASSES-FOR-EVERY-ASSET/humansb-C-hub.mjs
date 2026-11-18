export default (a) => {
  const hub = a.volumes["50"].body.find((s) => s.id === "hub");
  hub.profile = [[-0.025, 0], [-0.025, 0.055], [0.025, 0.055], [0.025, 0]];
  const g = a.representations["50"].geometry.front;
  Object.assign(g.find((s) => s.id === "hub-l"), { x: 0.25, w: 0.05 });
  Object.assign(g.find((s) => s.id === "hub-r"), { x: -0.3, w: 0.05 });
};

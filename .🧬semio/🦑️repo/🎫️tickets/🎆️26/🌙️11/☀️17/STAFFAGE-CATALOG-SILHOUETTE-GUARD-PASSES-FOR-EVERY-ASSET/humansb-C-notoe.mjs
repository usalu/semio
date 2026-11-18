export default (a) => { const g = a.representations["50"].geometry.plan; for (const id of ["toe-l","toe-r","foot-l","foot-r"]) g.splice(g.findIndex((s) => s.id === id), 1); };

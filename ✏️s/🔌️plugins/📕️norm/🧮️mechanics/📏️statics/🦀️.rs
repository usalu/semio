//! 📏️ Linear static analysis: model validation, reverse Cuthill-McKee numbering, skyline assembly, reactions, member end forces and sampled diagrams.

use crate::elements::*;
use crate::*;
use std::collections::BTreeMap;

const NONE: usize = usize::MAX;

#[derive(Clone, Debug)]
pub struct System {
    pub node_index: BTreeMap<String, usize>,
    pub member_index: BTreeMap<String, usize>,
    pub pos: Vec<usize>,
    pub fmap: Vec<usize>,
    pub owner: Vec<(usize, usize)>,
    pub n_free: usize,
    pub prescribed: Vec<f64>,
    pub spring: Vec<f64>,
    pub elements: Vec<Element>,
    pub ends: Vec<(usize, usize)>,
    pub profile: Vec<usize>,
}

pub struct MemberLoads {
    pub lines: Vec<([f64; 3], [f64; 3])>,
    pub thermal: Vec<(f64, f64, f64, f64)>,
}

pub struct Raw {
    pub u: Vec<f64>,
    pub forces: Vec<[f64; 12]>,
    pub loads: Vec<MemberLoads>,
    pub nodal: Vec<f64>,
    pub fsum: Vec<f64>,
}

fn invalid<T>(en: String, de: String) -> MechanicsResult<T> {
    Err(MechanicsError::invalid(en, de))
}

pub(crate) fn rcm(count: usize, ends: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![Vec::new(); count];
    for &(a, b) in ends {
        if a != b {
            adj[a].push(b);
            adj[b].push(a);
        }
    }
    for list in adj.iter_mut() {
        list.sort_unstable();
        list.dedup();
    }
    let degree: Vec<usize> = adj.iter().map(Vec::len).collect();
    let mut seen = vec![false; count];
    let mut order = Vec::with_capacity(count);
    let mut starts: Vec<usize> = (0..count).collect();
    starts.sort_by_key(|&i| (degree[i], i));
    for s in starts {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut queue = std::collections::VecDeque::from([s]);
        while let Some(v) = queue.pop_front() {
            order.push(v);
            let mut next: Vec<usize> = adj[v].iter().copied().filter(|&w| !seen[w]).collect();
            next.sort_by_key(|&w| (degree[w], w));
            for w in next {
                seen[w] = true;
                queue.push_back(w);
            }
        }
    }
    order.reverse();
    order
}

impl System {
    /// 🏁️ Validate the model and build numbering, element matrices and the skyline profile.
    pub fn build(model: &Model, control: &Control<'_>) -> MechanicsResult<Self> {
        let mut node_index = BTreeMap::new();
        for (i, n) in model.nodes.iter().enumerate() {
            if node_index.insert(n.id.clone(), i).is_some() {
                return invalid(format!("duplicate node {}", n.id), format!("doppelter Knoten {}", n.id));
            }
        }
        let materials: BTreeMap<&str, &Material> = model.materials.iter().map(|m| (m.id.as_str(), m)).collect();
        let sections: BTreeMap<&str, &Section> = model.sections.iter().map(|s| (s.id.as_str(), s)).collect();
        let mut member_index = BTreeMap::new();
        let mut elements = Vec::new();
        let mut ends = Vec::new();
        for (mi, m) in model.members.iter().enumerate() {
            control.check()?;
            if member_index.insert(m.id.clone(), mi).is_some() {
                return invalid(format!("duplicate member {}", m.id), format!("doppelter Stab {}", m.id));
            }
            let (Some(&a), Some(&b)) = (node_index.get(&m.node_i), node_index.get(&m.node_j)) else {
                return invalid(format!("member {} references an unknown node", m.id), format!("Stab {} verweist auf einen unbekannten Knoten", m.id));
            };
            let (Some(mat), Some(sec)) = (materials.get(m.material.as_str()), sections.get(m.section.as_str())) else {
                return invalid(format!("member {} references an unknown material or section", m.id), format!("Stab {} verweist auf einen unbekannten Werkstoff oder Querschnitt", m.id));
            };
            if !(mat.e > 0.0 && mat.g > 0.0 && sec.a > 0.0 && sec.iy > 0.0 && sec.iz > 0.0 && sec.j > 0.0) {
                return invalid(format!("member {} has non-positive material or section properties", m.id), format!("Stab {} hat nicht positive Werkstoff- oder Querschnittswerte", m.id));
            }
            let props = Props { e: mat.e, g: mat.g, rho: mat.rho, a: sec.a, iy: sec.iy, iz: sec.iz, j: sec.j, asy: sec.asy, asz: sec.asz, shear: m.shear_deformable };
            let (pa, pb) = (&model.nodes[a], &model.nodes[b]);
            elements.push(Element::new(props, [pa.x, pa.y, pa.z], [pb.x, pb.y, pb.z], m.roll, m.release_i, m.release_j).map_err(|e| match e {
                MechanicsError::Invalid(t) => MechanicsError::invalid(format!("member {}: {}", m.id, t.en), format!("Stab {}: {}", m.id, t.de)),
                other => other,
            })?);
            ends.push((a, b));
        }
        let nn = model.nodes.len();
        let order = rcm(nn, &ends);
        let mut pos = vec![0; nn];
        for (p, &n) in order.iter().enumerate() {
            pos[n] = p;
        }
        let mut fixed = vec![false; 6 * nn];
        let mut prescribed = vec![0.0; 6 * nn];
        let mut spring = vec![0.0; 6 * nn];
        for s in &model.supports {
            let Some(&n) = node_index.get(&s.node) else {
                return invalid(format!("support references unknown node {}", s.node), format!("Lager verweist auf unbekannten Knoten {}", s.node));
            };
            for k in 0..6 {
                let d = 6 * pos[n] + k;
                if s.fixed[k] {
                    fixed[d] = true;
                    prescribed[d] = s.prescribed[k];
                } else if s.spring[k] > 0.0 {
                    spring[d] += s.spring[k];
                }
            }
        }
        let mut fmap = vec![NONE; 6 * nn];
        let mut owner = Vec::new();
        for p in 0..nn {
            for k in 0..6 {
                if !fixed[6 * p + k] {
                    fmap[6 * p + k] = owner.len();
                    owner.push((order[p], k));
                }
            }
        }
        let n_free = owner.len();
        let mut profile: Vec<usize> = (0..n_free).collect();
        for &(a, b) in &ends {
            let dofs: Vec<usize> = (0..6).map(|k| fmap[6 * pos[a] + k]).chain((0..6).map(|k| fmap[6 * pos[b] + k])).filter(|&f| f != NONE).collect();
            if let Some(&lo) = dofs.iter().min() {
                for &f in &dofs {
                    profile[f] = profile[f].min(lo);
                }
            }
        }
        Ok(Self { node_index, member_index, pos, fmap, owner, n_free, prescribed, spring, elements, ends, profile })
    }

    pub fn dofs(&self, member: usize) -> [usize; 12] {
        let (a, b) = self.ends[member];
        let mut d = [0; 12];
        for k in 0..6 {
            d[k] = 6 * self.pos[a] + k;
            d[6 + k] = 6 * self.pos[b] + k;
        }
        d
    }

    pub fn unstable(&self, error: MechanicsError, model: &Model) -> MechanicsError {
        match error {
            MechanicsError::Singular { dof } if dof < self.owner.len() => {
                let (n, k) = self.owner[dof];
                MechanicsError::Unstable { node: model.nodes[n].id.clone(), dof: k }
            }
            other => other,
        }
    }

    /// 🧩️ Assemble a free-DOF skyline matrix from per-member local condensed matrices.
    pub fn assemble(&self, control: &Control<'_>, stage: &str, local: impl Fn(usize) -> M12) -> MechanicsResult<Skyline> {
        let mut sky = Skyline::new(self.profile.clone());
        for (mi, el) in self.elements.iter().enumerate() {
            if mi % 256 == 0 {
                control.check()?;
                control.report(stage, mi as f64 / self.elements.len().max(1) as f64);
            }
            let g = to_global(&el.r, &local(mi));
            let d = self.dofs(mi);
            for a in 0..12 {
                let fa = self.fmap[d[a]];
                if fa == NONE {
                    continue;
                }
                for b in a..12 {
                    let fb = self.fmap[d[b]];
                    if fb != NONE {
                        sky.add(fa, fb, 0.5 * (g[a][b] + g[b][a]));
                    }
                }
            }
        }
        Ok(sky)
    }

    pub fn expand(&self, free: &[f64]) -> Vec<f64> {
        (0..self.fmap.len()).map(|d| if self.fmap[d] == NONE { 0.0 } else { free[self.fmap[d]] }).collect()
    }

    pub fn displacements(&self, model: &Model, full: &[f64]) -> Vec<NodeDisplacement> {
        model.nodes.iter().enumerate().map(|(i, n)| NodeDisplacement { node: n.id.clone(), u: std::array::from_fn(|k| full[6 * self.pos[i] + k]) }).collect()
    }
}

pub struct Analysis<'m> {
    pub model: &'m Model,
    pub system: System,
    pub stiffness: Skyline,
    pub factor: Skyline,
    pub stations: usize,
}

impl<'m> Analysis<'m> {
    /// 🔧️ Assemble and factorise the stiffness once; load cases then cost one back-substitution each.
    pub fn new(model: &'m Model, control: &Control<'_>) -> MechanicsResult<Self> {
        control.report("validate", 0.0);
        let system = System::build(model, control)?;
        let mut stiffness = system.assemble(control, "assemble", |m| system.elements[m].k)?;
        for d in 0..system.fmap.len() {
            if system.fmap[d] != NONE && system.spring[d] > 0.0 {
                stiffness.add(system.fmap[d], system.fmap[d], system.spring[d]);
            }
        }
        let mut factor = stiffness.clone();
        factor.cholesky(control).map_err(|e| system.unstable(e, model))?;
        Ok(Self { model, system, stiffness, factor, stations: 21 })
    }

    pub fn member_loads(&self, case: &LoadCase) -> MechanicsResult<Vec<MemberLoads>> {
        let sys = &self.system;
        let mut out: Vec<MemberLoads> = self.model.members.iter().map(|_| MemberLoads { lines: Vec::new(), thermal: Vec::new() }).collect();
        let rot = |m: usize, v: [f64; 3]| -> [f64; 3] { std::array::from_fn(|i| (0..3).map(|j| sys.elements[m].r[i][j] * v[j]).sum()) };
        for d in &case.distributed {
            let Some(&m) = sys.member_index.get(&d.member) else {
                return invalid(format!("load references unknown member {}", d.member), format!("Last verweist auf unbekannten Stab {}", d.member));
            };
            let (s, e) = if d.frame == LoadFrame::Global { (rot(m, d.start), rot(m, d.end)) } else { (d.start, d.end) };
            out[m].lines.push((s, e));
        }
        for t in &case.temperature {
            let Some(&m) = sys.member_index.get(&t.member) else {
                return invalid(format!("temperature load references unknown member {}", t.member), format!("Temperaturlast verweist auf unbekannten Stab {}", t.member));
            };
            out[m].thermal.push((t.alpha, t.uniform, t.gradient_y, t.gradient_z));
        }
        if case.acceleration != [0.0; 3] {
            for (m, el) in sys.elements.iter().enumerate() {
                let w = rot(m, case.acceleration.map(|a| a * el.props.rho * el.props.a));
                out[m].lines.push((w, w));
            }
        }
        Ok(out)
    }

    /// 🧾️ Solve one load case into raw global displacements and local member end forces.
    pub fn solve_raw(&self, case: &LoadCase, control: &Control<'_>) -> MechanicsResult<Raw> {
        let sys = &self.system;
        let mut nodal = vec![0.0; sys.fmap.len()];
        for l in &case.nodal {
            let Some(&n) = sys.node_index.get(&l.node) else {
                return invalid(format!("load references unknown node {}", l.node), format!("Last verweist auf unbekannten Knoten {}", l.node));
            };
            for k in 0..6 {
                nodal[6 * sys.pos[n] + k] += l.force[k];
            }
        }
        let loads = self.member_loads(case)?;
        let mut rhs: Vec<f64> = vec![0.0; sys.n_free];
        for d in 0..nodal.len() {
            if sys.fmap[d] != NONE {
                rhs[sys.fmap[d]] += nodal[d];
            }
        }
        let mut local_f = Vec::with_capacity(loads.len());
        for (m, el) in sys.elements.iter().enumerate() {
            control.check()?;
            let f = el.load_vector(&loads[m].lines, &loads[m].thermal);
            let fg = rotate(&el.r, &f, true);
            let d = sys.dofs(m);
            let mut up = [0.0; 12];
            for a in 0..12 {
                up[a] = if sys.fmap[d[a]] == NONE { sys.prescribed[d[a]] } else { 0.0 };
            }
            let kp = mat_vec(&to_global(&el.r, &el.k), &up);
            for a in 0..12 {
                if sys.fmap[d[a]] != NONE {
                    rhs[sys.fmap[d[a]]] += fg[a] - kp[a];
                }
            }
            local_f.push(f);
        }
        control.report("solve", 0.5);
        self.factor.solve(&mut rhs);
        let mut u = sys.expand(&rhs);
        for d in 0..u.len() {
            if sys.fmap[d] == NONE {
                u[d] = sys.prescribed[d];
            }
        }
        let mut forces = Vec::with_capacity(loads.len());
        let mut fsum = vec![0.0; u.len()];
        for (m, el) in sys.elements.iter().enumerate() {
            let d = sys.dofs(m);
            let ug: [f64; 12] = std::array::from_fn(|a| u[d[a]]);
            let ul = rotate(&el.r, &ug, false);
            let kf = mat_vec(&el.k, &ul);
            let f: [f64; 12] = std::array::from_fn(|a| kf[a] - local_f[m][a]);
            let fg = rotate(&el.r, &f, true);
            for a in 0..12 {
                fsum[d[a]] += fg[a];
            }
            forces.push(f);
        }
        control.report("solve", 1.0);
        Ok(Raw { u, forces, loads, nodal, fsum })
    }

    /// 📊️ Solve one load case into displacements, reactions, end forces and M/V/N/T diagrams.
    pub fn solve_case(&self, case: &LoadCase, control: &Control<'_>) -> MechanicsResult<StaticResult> {
        let raw = self.solve_raw(case, control)?;
        let sys = &self.system;
        let mut reactions = Vec::new();
        for s in &self.model.supports {
            let n = sys.node_index[&s.node];
            let force = std::array::from_fn(|k| {
                let d = 6 * sys.pos[n] + k;
                if sys.fmap[d] == NONE { raw.fsum[d] - raw.nodal[d] } else { -sys.spring[d] * raw.u[d] }
            });
            reactions.push(Reaction { node: s.node.clone(), force });
        }
        let members = self.model.members.iter().enumerate().map(|(m, mem)| diagram(&mem.id, sys.elements[m].l, &raw.forces[m], &raw.loads[m], self.stations)).collect();
        Ok(StaticResult { case: case.id.clone(), displacements: sys.displacements(self.model, &raw.u), reactions, members })
    }

    /// 🗂️ Solve every load case of the model in order.
    pub fn solve_all(&self, control: &Control<'_>) -> MechanicsResult<Vec<StaticResult>> {
        let count = self.model.load_cases.len().max(1);
        let mut out = Vec::new();
        for (i, case) in self.model.load_cases.iter().enumerate() {
            control.check()?;
            control.report("cases", i as f64 / count as f64);
            out.push(self.solve_case(case, control)?);
        }
        Ok(out)
    }

    pub fn axial_forces(&self, raw: &Raw) -> Vec<f64> {
        raw.forces.iter().map(|f| 0.5 * (-f[0] + f[6])).collect()
    }

    /// 🌊️ Global displacement offsets along a member (Hermite bending, linear axial) for a displacement field such as a mode shape.
    pub fn member_curve(&self, shape: &[NodeDisplacement], member: &str, samples: usize) -> MechanicsResult<Vec<[f64; 3]>> {
        let sys = &self.system;
        let Some(&m) = sys.member_index.get(member) else {
            return invalid(format!("unknown member {member}"), format!("unbekannter Stab {member}"));
        };
        let lookup: BTreeMap<&str, &Dof6> = shape.iter().map(|s| (s.node.as_str(), &s.u)).collect();
        let mem = &self.model.members[m];
        let (Some(a), Some(b)) = (lookup.get(mem.node_i.as_str()), lookup.get(mem.node_j.as_str())) else {
            return invalid(format!("shape misses an end node of {member}"), format!("Form fehlt ein Endknoten von {member}"));
        };
        let el = &sys.elements[m];
        let ug: [f64; 12] = std::array::from_fn(|i| if i < 6 { a[i] } else { b[i - 6] });
        let u = mat_vec(&el.t, &rotate(&el.r, &ug, false));
        let l = el.l;
        Ok((0..samples.max(2))
            .map(|i| {
                let s = i as f64 / (samples.max(2) - 1) as f64;
                let h = [1.0 - 3.0 * s * s + 2.0 * s.powi(3), l * (s - 2.0 * s * s + s.powi(3)), 3.0 * s * s - 2.0 * s.powi(3), l * (-s * s + s.powi(3))];
                let ux = (1.0 - s) * u[0] + s * u[6];
                let uy = h[0] * u[1] + h[1] * u[5] + h[2] * u[7] + h[3] * u[11];
                let uz = h[0] * u[2] - h[1] * u[4] + h[2] * u[8] - h[3] * u[10];
                std::array::from_fn(|k| el.r[0][k] * ux + el.r[1][k] * uy + el.r[2][k] * uz)
            })
            .collect())
    }
}

fn diagram(id: &str, l: f64, f: &[f64; 12], loads: &MemberLoads, stations: usize) -> MemberForces {
    let count = stations.max(2);
    let diagram = (0..count)
        .map(|i| {
            let x = l * i as f64 / (count - 1) as f64;
            let (mut i1, mut i2) = ([0.0; 3], [0.0; 3]);
            for (w0, w1) in &loads.lines {
                for k in 0..3 {
                    let dw = w1[k] - w0[k];
                    i1[k] += w0[k] * x + dw * x * x / (2.0 * l);
                    i2[k] += -w0[k] * x * x / 2.0 - dw * x.powi(3) / (6.0 * l);
                }
            }
            DiagramPoint { x, n: -f[0] - i1[0], vy: -f[1] - i1[1], vz: -f[2] - i1[2], t: -f[3], my: -f[4] - x * f[2] + i2[2], mz: -f[5] + x * f[1] - i2[1] }
        })
        .collect();
    MemberForces { member: id.to_string(), length: l, end_i: std::array::from_fn(|k| f[k]), end_j: std::array::from_fn(|k| f[6 + k]), diagram }
}

impl Model {
    /// ✂️ Split every member into `segments` equal elements with interpolated line loads so stability and modal analyses resolve curvature.
    pub fn refine(&self, segments: usize) -> Model {
        let n = segments.max(1);
        if n == 1 {
            return self.clone();
        }
        let mut out = self.clone();
        out.members.clear();
        out.load_cases.iter_mut().for_each(|c| {
            c.distributed.clear();
            c.temperature.clear();
        });
        let pos: BTreeMap<&str, &Node> = self.nodes.iter().map(|x| (x.id.as_str(), x)).collect();
        for m in &self.members {
            let (a, b) = (pos[m.node_i.as_str()], pos[m.node_j.as_str()]);
            let name = |k: usize| if k == 0 { m.node_i.clone() } else if k == n { m.node_j.clone() } else { format!("{}@{k}", m.id) };
            for k in 1..n {
                let t = k as f64 / n as f64;
                out.nodes.push(Node { id: name(k), x: a.x + t * (b.x - a.x), y: a.y + t * (b.y - a.y), z: a.z + t * (b.z - a.z) });
            }
            for k in 0..n {
                out.members.push(Member {
                    id: format!("{}#{}", m.id, k + 1),
                    node_i: name(k),
                    node_j: name(k + 1),
                    release_i: if k == 0 { m.release_i } else { [false; 6] },
                    release_j: if k == n - 1 { m.release_j } else { [false; 6] },
                    ..m.clone()
                });
            }
        }
        for (ci, c) in self.load_cases.iter().enumerate() {
            for d in &c.distributed {
                for k in 0..n {
                    let lerp = |t: f64| std::array::from_fn(|i| d.start[i] + t * (d.end[i] - d.start[i]));
                    out.load_cases[ci].distributed.push(DistributedLoad { member: format!("{}#{}", d.member, k + 1), frame: d.frame, start: lerp(k as f64 / n as f64), end: lerp((k + 1) as f64 / n as f64) });
                }
            }
            for t in &c.temperature {
                for k in 0..n {
                    out.load_cases[ci].temperature.push(TemperatureLoad { member: format!("{}#{}", t.member, k + 1), ..t.clone() });
                }
            }
        }
        out
    }
}

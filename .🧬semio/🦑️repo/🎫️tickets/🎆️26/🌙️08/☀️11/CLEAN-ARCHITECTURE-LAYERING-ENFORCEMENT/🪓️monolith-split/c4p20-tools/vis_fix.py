import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p20 as P
from specs.lib import EN
M = os.path.join(P.REPO, EN, '🥽️mesh')

def edit(path, rows):
    t = open(path, encoding='utf-8').read()
    lines = t.split('\n')
    for ln, old, new in rows:
        assert lines[ln - 1] == old, (path[-60:], ln, lines[ln - 1], old)
        lines[ln - 1] = new
    open(path, 'w', encoding='utf-8').write('\n'.join(lines))

repair = os.path.join(M, '🩹️repair/⏱️bounded-preparations/🦀️.rs')
edit(repair, [
    (58, '    pub(super) fn new(mesh: &TriMesh, minimum_component_faces: usize, minimum_component_bbox_fraction: f64, keep_largest_component: bool) -> Self {', '    pub fn new(mesh: &TriMesh, minimum_component_faces: usize, minimum_component_bbox_fraction: f64, keep_largest_component: bool) -> Self {'),
    (96, '    fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {', '    pub fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {'),
    (242, '    fn finish(self) -> TriMesh {', '    pub fn finish(self) -> TriMesh {'),
    (248, 'struct BoundedEdgeRecord {', 'pub struct BoundedEdgeRecord {'),
    (249, '    directed: [u32; 2],', '    pub directed: [u32; 2],'),
    (250, '    pub(super) faces: Vec<u32>,', '    pub faces: Vec<u32>,'),
    (256, '    records: BTreeMap<(u32, u32), BoundedEdgeRecord>,', '    pub records: BTreeMap<(u32, u32), BoundedEdgeRecord>,'),
    (260, '    pub(super) fn new() -> Self {', '    pub fn new() -> Self {'),
    (264, '    fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {', '    pub fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {'),
    (312, '    pub(super) fn new(mesh: &TriMesh) -> Self {', '    pub fn new(mesh: &TriMesh) -> Self {'),
    (316, '    fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {', '    pub fn advance(&mut self, mesh: &TriMesh, item_budget: usize) -> bool {'),
    (397, '    fn finish(self, mesh: &mut TriMesh) {', '    pub fn finish(self, mesh: &mut TriMesh) {'),
    (421, '    pub(super) fn new() -> Self {', '    pub fn new() -> Self {'),
    (425, '    fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {', '    pub fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {'),
    (511, '    pub(super) fn new(max_boundary_verts: usize) -> Self {', '    pub fn new(max_boundary_verts: usize) -> Self {'),
    (515, '    fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {', '    pub fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {'),
    (629, '    fn exceeded(&self) -> bool {', '    pub fn exceeded(&self) -> bool {'),
])
trim = os.path.join(M, '🔺️surface/📐️trimesh-and-vectors/🦀️.rs')
edit(trim, [
    (191, '    pub(super) fn new(n: usize) -> Self {', '    pub fn new(n: usize) -> Self {'),
    (195, '    pub(super) fn find(&mut self, x: u32) -> u32 {', '    pub fn find(&mut self, x: u32) -> u32 {'),
    (209, '    fn union(&mut self, a: u32, b: u32) {', '    pub fn union(&mut self, a: u32, b: u32) {'),
])
iso = os.path.join(M, '🔺️surface/🧊️iso-surface-extraction/🦀️.rs')
edit(iso, [
    (633, '    fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {', '    pub fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {'),
])
print('vis fixes applied')

from specs.lib import *

SPEC = dict(
    orig='surface.rs.txt',
    file=EN + '/🥽️mesh/🔺️surface/🦀️.rs',
    nest=False,
    external=[],
    vis_edits=[
        ('📐️trimesh-and-vectors', 191, '    pub(super) fn new(n: usize) -> Self {', '    pub fn new(n: usize) -> Self {'),
        ('📐️trimesh-and-vectors', 195, '    pub(super) fn find(&mut self, x: u32) -> u32 {', '    pub fn find(&mut self, x: u32) -> u32 {'),
        ('📐️trimesh-and-vectors', 209, '    fn union(&mut self, a: u32, b: u32) {', '    pub fn union(&mut self, a: u32, b: u32) {'),
        ('🧊️iso-surface-extraction', 633, '    fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {', '    pub fn index(&self, i: i32, j: i32, k: i32) -> Option<usize> {'),
    ],
    children=[
        ch('📐', 'trimesh-and-vectors', 'trimesh_and_vectors', [(10, 222)],
           'Triangle mesh of the mesh surface: the shared 3-vector helpers, the indexed triangle mesh with its edge map and the disjoint set.'),
        ch('🕸', 'halfedge-topology', 'halfedge_topology', [(228, 385)],
           'Half-edge topology of the mesh surface: the topology error and the half-edge structure with its manifold, boundary and neighbour queries.'),
        ch('🧊', 'iso-surface-extraction', 'iso_surface_extraction', [(391, 1065)],
           'Iso-surface extraction of the mesh surface: the marching tetrahedra over a TSDF volume with its bounded resumable preparation and the extraction from a dense field.'),
    ],
)

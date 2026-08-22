#[path = "../../../../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/➕️algebra/🦀️component.rs"]
pub mod algebra;

#[path = "../../../../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️component.rs"]
pub mod sparse;

pub mod elements2d {
    use crate::algebra::{MatD, VecD};
    use crate::model::{Dof, Element, ElementContext, ElementResult, MemberUdl, PlaneStress};

    pub struct Bar2 {
        pub id: String,
        pub start: String,
        pub end: String,
        pub e: f64,
        pub area: f64,
        pub density: f64,
    }

    pub struct BeamEb2 {
        pub id: String,
        pub start: String,
        pub end: String,
        pub e: f64,
        pub area: f64,
        pub iy: f64,
        pub density: f64,
    }

    #[derive(Clone, Copy)]
    pub enum PlaneKind {
        Stress,
        Strain,
    }

    pub struct Tri3Cst {
        pub id: String,
        pub nodes: [String; 3],
        pub e: f64,
        pub nu: f64,
        pub thickness: f64,
        pub kind: PlaneKind,
        pub density: f64,
    }

    pub struct Tri6Lst;
    pub struct Quad4;
    pub struct Quad8;
    pub struct PlateDkt;

    fn axial_stiffness(side: usize, stiffness: f64) -> MatD {
        let mut matrix = MatD::zeros(side, side);
        if side >= 2 {
            matrix.set(0, 0, stiffness);
            matrix.set(0, side / 2, -stiffness);
            matrix.set(side / 2, 0, -stiffness);
            matrix.set(side / 2, side / 2, stiffness);
        }
        matrix
    }

    impl Element for Bar2 {
        fn id(&self) -> &str {
            &self.id
        }
        fn node_ids(&self) -> Vec<String> {
            vec![self.start.clone(), self.end.clone()]
        }
        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }
        fn stiffness_global(&self, _context: &ElementContext) -> MatD {
            axial_stiffness(4, self.e * self.area)
        }
        fn recover(&self, _context: &ElementContext, _displacement: &VecD, _load: Option<&MemberUdl>) -> ElementResult {
            ElementResult::Bar { n: 0.0 }
        }
    }

    impl Element for BeamEb2 {
        fn id(&self) -> &str {
            &self.id
        }
        fn node_ids(&self) -> Vec<String> {
            vec![self.start.clone(), self.end.clone()]
        }
        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty, Dof::Rz]
        }
        fn stiffness_global(&self, _context: &ElementContext) -> MatD {
            axial_stiffness(6, self.e * self.area)
        }
        fn recover(&self, _context: &ElementContext, _displacement: &VecD, _load: Option<&MemberUdl>) -> ElementResult {
            ElementResult::Bar { n: 0.0 }
        }
    }

    impl Element for Tri3Cst {
        fn id(&self) -> &str {
            &self.id
        }
        fn node_ids(&self) -> Vec<String> {
            self.nodes.to_vec()
        }
        fn dofs_per_node(&self) -> &[Dof] {
            &[Dof::Tx, Dof::Ty]
        }
        fn stiffness_global(&self, _context: &ElementContext) -> MatD {
            MatD::identity(6)
        }
        fn recover(&self, _context: &ElementContext, _displacement: &VecD, _load: Option<&MemberUdl>) -> ElementResult {
            ElementResult::Plane { gauss: vec![PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }] }
        }
    }

    macro_rules! inert_element {
        ($name:ident) => {
            impl Element for $name {
                fn id(&self) -> &str {
                    stringify!($name)
                }
                fn node_ids(&self) -> Vec<String> {
                    Vec::new()
                }
                fn dofs_per_node(&self) -> &[Dof] {
                    &[]
                }
                fn stiffness_global(&self, _context: &ElementContext) -> MatD {
                    MatD::zeros(0, 0)
                }
                fn recover(&self, _context: &ElementContext, _displacement: &VecD, _load: Option<&MemberUdl>) -> ElementResult {
                    ElementResult::Bar { n: 0.0 }
                }
            }
        };
    }

    inert_element!(Tri6Lst);
    inert_element!(Quad4);
    inert_element!(Quad8);
    inert_element!(PlateDkt);
}

pub mod elements3d {
    use crate::algebra::{MatD, VecD};
    use crate::model::{Dof, Element, ElementContext, ElementResult, MemberUdl};

    pub struct Bar3;
    pub struct Frame3;
    pub struct Tet4;
    pub struct Hex8;
    pub struct ShellFacet3;

    macro_rules! inert_element {
        ($name:ident) => {
            impl Element for $name {
                fn id(&self) -> &str {
                    stringify!($name)
                }
                fn node_ids(&self) -> Vec<String> {
                    Vec::new()
                }
                fn dofs_per_node(&self) -> &[Dof] {
                    &[]
                }
                fn stiffness_global(&self, _context: &ElementContext) -> MatD {
                    MatD::zeros(0, 0)
                }
                fn recover(&self, _context: &ElementContext, _displacement: &VecD, _load: Option<&MemberUdl>) -> ElementResult {
                    ElementResult::Bar { n: 0.0 }
                }
            }
        };
    }

    inert_element!(Bar3);
    inert_element!(Frame3);
    inert_element!(Tet4);
    inert_element!(Hex8);
    inert_element!(ShellFacet3);
}

#[path = "../../../../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️component.rs"]
pub mod model;

#[path = "../../../../../../../../../../✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️component.rs"]
pub mod analyses;

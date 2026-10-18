//! 🕸️ Paged mesh ownership: the exclusive authority, write tokens, leases, native resource custody and the instance and component source records drawn from them.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

#[path = "../♻️retirement/🦀️.rs"]
mod mesh3d_retirement;
use mesh3d_retirement::{IneligibleGroups, NumericSeen};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress},
    retirement::controlled::ControlledRetirement,
    RetirementDemand,
};
use semio_framework_geometry::{Mat4, Vec3};
pub use semio_framework_mesh_component_references::ComponentReferenceTable;
use semio_framework_ui_scene_algebra::{mat4_from_quat_m, mat4_scale_vec_m, mat4_translation_m, vec3_from_array_m, Mat4Math};

//#region Mesh
pub const MESH3D_AUTHORITY_CAPACITY: usize = 256;
pub const MESH3D_PAGE_BYTES: usize = 16 * 1024;
pub const MESH3D_OWNER_PAGE_CAPACITY: usize = 1_024;
pub const MESH3D_AUTHORITY_PAGE_CAPACITY: usize = 4_096;
pub const MESH3D_OWNER_BYTE_CAPACITY: usize = MESH3D_PAGE_BYTES * MESH3D_OWNER_PAGE_CAPACITY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mesh3dField {
    Positions,
    Normals,
    Indices,
    FaceIds,
    VertexIds,
    Edges,
    EdgeIds,
    Uvs,
    Colors,
    UvsMetallicRoughness,
    UvsNormal,
    UvsOcclusion,
    UvsEmissive,
    Tangents,
}

impl Mesh3dField {
    fn index(self) -> usize {
        match self {
            Self::Positions => 0,
            Self::Normals => 1,
            Self::Indices => 2,
            Self::FaceIds => 3,
            Self::VertexIds => 4,
            Self::Edges => 5,
            Self::EdgeIds => 6,
            Self::Uvs => 7,
            Self::Colors => 8,
            Self::UvsMetallicRoughness=>9,Self::UvsNormal=>10,Self::UvsOcclusion=>11,Self::UvsEmissive=>12,Self::Tangents=>13,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesh3dSchema {
    pub vertices: u32,
    pub indices: u32,
    pub face_ids: u32,
    pub vertex_ids: u32,
    pub edges: u32,
    pub edge_ids: u32,
    pub uvs: u32,
    pub colors: u32,
    pub surface_uvs:[u32;4],
    pub tangents:u32,
}

impl Mesh3dSchema {
    pub fn triangle_mesh(vertices: u32, indices: u32) -> Self {
        Self { vertices, indices, face_ids: 0, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0,surface_uvs:[0;4],tangents:0 }
    }

    fn field_items(self, field: Mesh3dField) -> u32 {
        match field {
            Mesh3dField::Positions | Mesh3dField::Normals => self.vertices,
            Mesh3dField::Indices => self.indices,
            Mesh3dField::FaceIds => self.face_ids,
            Mesh3dField::VertexIds => self.vertex_ids,
            Mesh3dField::Edges => self.edges,
            Mesh3dField::EdgeIds => self.edge_ids,
            Mesh3dField::Uvs => self.uvs,
            Mesh3dField::Colors => self.colors,Mesh3dField::Tangents=>self.tangents,
            Mesh3dField::UvsMetallicRoughness=>self.surface_uvs[0],Mesh3dField::UvsNormal=>self.surface_uvs[1],Mesh3dField::UvsOcclusion=>self.surface_uvs[2],Mesh3dField::UvsEmissive=>self.surface_uvs[3],
        }
    }

    fn field_item_bytes(field: Mesh3dField) -> usize {
        match field {
            Mesh3dField::Positions | Mesh3dField::Normals => 12,
            Mesh3dField::Edges => 24,
            Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive => 8,
            Mesh3dField::Colors | Mesh3dField::Tangents => 16,
            Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds => 4,
        }
    }

    /// 🧮️ Refuses a schema whose field counts disagree or whose packed geometry exceeds the mesh owner byte capacity.
    pub fn validate(self) -> Result<(), Mesh3dFault> {
        self.layout().map(|_| ())
    }

    fn layout(self) -> Result<Mesh3dLayout, Mesh3dFault> {
        if (self.vertices == 0 && (self.edges == 0 || self.indices != 0))
            || (self.indices == 0 && self.edges == 0 && self.vertex_ids == 0)
            || !self.indices.is_multiple_of(3)
            || (self.face_ids != 0 && self.face_ids != self.indices / 3)
            || (self.vertex_ids != 0 && self.vertex_ids != self.vertices)
            || (self.edge_ids != 0 && self.edge_ids != self.edges)
            || (self.uvs != 0 && self.uvs != self.vertices)
            || self.surface_uvs.iter().any(|count|*count!=0 && *count!=self.vertices)
            || (self.colors != 0 && self.colors != self.vertices)
            || (self.tangents != 0 && self.tangents != self.vertices)
        {
            return Err(Mesh3dFault::Schema);
        }
        let mut offsets = [0usize; 14];
        let mut total = 0usize;
        for field in [Mesh3dField::Positions, Mesh3dField::Normals, Mesh3dField::Indices, Mesh3dField::FaceIds, Mesh3dField::VertexIds, Mesh3dField::Edges, Mesh3dField::EdgeIds, Mesh3dField::Uvs, Mesh3dField::Colors,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive,Mesh3dField::Tangents] {
            offsets[field.index()] = total;
            let bytes = usize::try_from(self.field_items(field)).ok().and_then(|items| items.checked_mul(Self::field_item_bytes(field))).ok_or(Mesh3dFault::ByteCapacity)?;
            total = total.checked_add(bytes).ok_or(Mesh3dFault::ByteCapacity)?;
        }
        if total > MESH3D_OWNER_BYTE_CAPACITY {
            return Err(Mesh3dFault::ByteCapacity);
        }
        Ok(Mesh3dLayout { offsets, total_bytes: total, page_count: total.div_ceil(MESH3D_PAGE_BYTES) as u16 })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mesh3dFault {
    Busy,
    Closing,
    ItemCapacity,
    PageCapacity,
    ByteCapacity,
    Schema,
    Stale,
    Order,
    Incomplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mesh3dWriteToken {
    slot: u16,
    epoch: u64,
    generation: u64,
    revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Mesh3dLease {
    slot: u16,
    epoch: u64,
    generation: u64,
    revision: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mesh3dComponentAdmissionStep {
    pub complete: bool,
    pub processed_items: usize,
    pub processed_bytes: usize,
    pub retired_items: usize,
    pub retired_bytes: usize,
    pub ownership: RetainedCloneProgress,
}

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct Mesh3dOwnershipStep { pub complete:bool,pub progress:RetainedCloneProgress }

fn mesh3d_component_number(label: &str) -> Option<u64> {
    if label.len() > 20 || !label.as_bytes().first().is_some_and(|byte| matches!(byte, b'1'..=b'9')) || !label.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
    label.parse().ok()
}

#[derive(Clone, Copy)]
struct Mesh3dLayout {
    offsets: [usize; 14],
    total_bytes: usize,
    page_count: u16,
}

struct Mesh3dPage {
    bytes: Box<[u8; MESH3D_PAGE_BYTES]>,
}

fn component_lengths(table:&ComponentReferenceTable)->Result<[usize;3],Mesh3dFault> {
    if table.len()>3 {return Err(Mesh3dFault::Schema);}
    let mut lengths=[0;3];
    let mut domains=0u8;
    for(kind,labels)in table {let domain=match kind.as_str(){"face"=>0,"edge"=>1,"vertex"=>2,_=>return Err(Mesh3dFault::Schema)};if labels.len()>600_000||domains&(1<<domain)!=0{return Err(Mesh3dFault::Schema);}domains|=1<<domain;lengths[domain]=labels.len();}
    Ok(lengths)
}

fn controlled_demands<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(|_|Mesh3dFault::Closing)?,capacity_bytes:owner.next_capacity_byte_demand(work).map_err(|_|Mesh3dFault::Closing)?,release_bytes:owner.next_release_byte_demand().map_err(|_|Mesh3dFault::Closing)?,depth:owner.next_depth_demand().map_err(|_|Mesh3dFault::Closing)?})
}

struct Mesh3dOwner {
    generation: u64,
    revision: u64,
    schema: Mesh3dSchema,
    layout: Mesh3dLayout,
    pages: Option<Box<[Option<Mesh3dPage>; MESH3D_OWNER_PAGE_CAPACITY]>>,
    allocated_pages: u16,
    written: [u32; 14],
    aabb_min: [f32; 3],
    aabb_max: [f32; 3],
    closing: bool,
    close_page: u16,
    component_references: Option<ComponentReferenceTable>,
    component_retirement: Option<ControlledRetirement<ComponentReferenceTable>>,
    scratch_retirement: Option<ControlledRetirement<(NumericSeen,IneligibleGroups)>>,
    component_domain: u8,
    component_group: usize,
    component_complete: bool,
    component_credit: usize,
    component_fault: Option<Mesh3dFault>,
    component_seen: NumericSeen,
    component_ineligible: IneligibleGroups,
    component_probe: Option<(u64,usize)>,
    has_topological_vertex: bool,
    resource_claimed: bool,
    resource_closing: bool,
    resource_loans: u32,
}

impl Mesh3dOwner {
    fn new(generation: u64, revision: u64, schema: Mesh3dSchema, layout: Mesh3dLayout) -> Self {
        Self { generation, revision, schema, layout, pages: Some(Box::new(std::array::from_fn(|_| None))), allocated_pages: 0, written: [0; 14], aabb_min: [f32::INFINITY; 3], aabb_max: [f32::NEG_INFINITY; 3], closing: false, close_page: 0, component_references: None, component_retirement: None,scratch_retirement:None, component_domain: 0, component_group: 0, component_complete: true, component_credit: 0, component_fault: None, component_seen: NumericSeen::default(), component_ineligible: IneligibleGroups::default(),component_probe:None, has_topological_vertex: false, resource_claimed: false, resource_closing: false, resource_loans: 0 }
    }

    fn component_header_credit(&self) -> Result<usize, Mesh3dFault> {
        let Some(table) = self.component_references.as_ref() else { return Ok(0); };
        if table.len() > 3 { return Err(Mesh3dFault::Schema); }
        let credit = table.directory_capacity_bytes().ok_or(Mesh3dFault::ByteCapacity)?;
        for (kind, labels) in table {
            if !matches!(kind.as_str(), "face" | "edge" | "vertex") || labels.len() > 600_000 { return Err(Mesh3dFault::Schema); }
        }
        let lengths=component_lengths(table)?;
        credit.checked_add(NumericSeen::capacity_bytes(*lengths.iter().max().unwrap()).ok_or(Mesh3dFault::ByteCapacity)?).and_then(|credit|credit.checked_add(lengths.iter().sum::<usize>())).ok_or(Mesh3dFault::ByteCapacity)
    }

    fn component_current_label(&self) -> Option<&String> {
        self.component_references.as_ref()?.get(*["face", "edge", "vertex"].get(self.component_domain as usize)?)?.get(self.component_group)
    }

    fn component_key(&self, group: u32) -> u64 { (u64::from(self.component_domain) << 32) | u64::from(group) }

    fn component_turn_byte_demand(&self) -> usize {
        if self.component_probe.is_some() {return 12;}
        if let Some(label) = self.component_current_label() { return 24 + if label.len() <= 20 { label.len() } else { 0 }; }
        if self.component_seen.is_empty() { 1 } else { 12 }
    }

    fn component_turn_credit(&self) -> Result<(usize, Option<u64>), Mesh3dFault> {
        if self.component_probe.is_some() {return Ok((self.component_credit,None));}
        if let Some(label) = self.component_current_label() {
            let credit = self.component_credit.checked_add(label.capacity()).ok_or(Mesh3dFault::ByteCapacity)?;
            let number = mesh3d_component_number(label);
            return Ok((credit, number));
        }
        Ok((self.component_credit, None))
    }

    fn component_turn(&mut self, number: Option<u64>) -> Mesh3dComponentAdmissionStep {
        if let Some((number,probe))=self.component_probe {
            self.component_probe_turn(number,probe);
            return Mesh3dComponentAdmissionStep{processed_items:1,processed_bytes:12,..Default::default()};
        }
        if let Some(label) = self.component_current_label() {
            let bytes = 24 + if label.len() <= 20 { label.len() } else { 0 };
            let group = self.component_group as u32;
            match number {
                Some(number) => self.component_probe_turn(number,self.component_seen.first_probe(number)),
                None => { self.component_ineligible.insert(self.component_key(group));self.component_group+=1; }
            }
            return Mesh3dComponentAdmissionStep { processed_items: 1, processed_bytes: bytes, ..Default::default() };
        }
        if self.component_seen.pop_first().is_some() { return Mesh3dComponentAdmissionStep { retired_items: 1, retired_bytes: 12, ..Default::default() }; }
        self.component_domain += 1;
        self.component_group = 0;
        self.component_complete = self.component_domain == 3;
        Mesh3dComponentAdmissionStep { complete: self.component_complete, processed_items: 1, processed_bytes: 1, ..Default::default() }
    }

    fn component_probe_turn(&mut self,number:u64,probe:usize) {
        let group=self.component_group as u32;
        match self.component_seen.probe(probe) {
            None=>{self.component_seen.insert_at(probe,number,group);self.component_probe=None;self.component_group+=1;},
            Some((previous,first))if previous==number=>{self.component_ineligible.insert(self.component_key(first));self.component_ineligible.insert(self.component_key(group));self.component_probe=None;self.component_group+=1;},
            Some(_)=>self.component_probe=Some((number,self.component_seen.next_probe(probe))),
        }
    }

    fn allocate_step(&mut self,grant:RetainedCloneGrant) -> Mesh3dOwnershipStep {
        if self.allocated_pages == self.layout.page_count {
            return Mesh3dOwnershipStep{complete:true,..Default::default()};
        }
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<MESH3D_PAGE_BYTES{return Mesh3dOwnershipStep::default();}
        let slot = usize::from(self.allocated_pages);
        self.pages.as_mut().unwrap()[slot] = Some(Mesh3dPage { bytes: Box::new([0; MESH3D_PAGE_BYTES]) });
        self.allocated_pages += 1;
        Mesh3dOwnershipStep{complete:self.allocated_pages==self.layout.page_count,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:MESH3D_PAGE_BYTES,..Default::default()}}
    }

    fn write(&mut self, field: Mesh3dField, bytes: &[u8]) -> Result<u32, Mesh3dFault> {
        if self.closing || self.allocated_pages != self.layout.page_count {
            return Err(if self.closing { Mesh3dFault::Closing } else { Mesh3dFault::Incomplete });
        }
        let field_index = field.index();
        let item = self.written[field_index];
        if item >= self.schema.field_items(field) || bytes.len() != Mesh3dSchema::field_item_bytes(field) {
            return Err(Mesh3dFault::Order);
        }
        let absolute = self.layout.offsets[field_index].checked_add(item as usize * bytes.len()).ok_or(Mesh3dFault::ByteCapacity)?;
        self.write_at(absolute, bytes)?;
        self.written[field_index] += 1;
        if field == Mesh3dField::VertexIds && bytes != u32::MAX.to_le_bytes() { self.has_topological_vertex = true; }
        Ok(item)
    }

    fn write_at(&mut self, absolute: usize, bytes: &[u8]) -> Result<(), Mesh3dFault> {
        let end = absolute.checked_add(bytes.len()).ok_or(Mesh3dFault::ByteCapacity)?;
        if end > self.layout.total_bytes {
            return Err(Mesh3dFault::ByteCapacity);
        }
        for (delta, byte) in bytes.iter().enumerate() {
            let at = absolute + delta;
            let page = self.pages.as_mut().ok_or(Mesh3dFault::Incomplete)?[at / MESH3D_PAGE_BYTES].as_mut().ok_or(Mesh3dFault::Incomplete)?;
            page.bytes[at % MESH3D_PAGE_BYTES] = *byte;
        }
        Ok(())
    }

    fn read<const N: usize>(&self, absolute: usize) -> Result<[u8; N], Mesh3dFault> {
        let end = absolute.checked_add(N).ok_or(Mesh3dFault::ByteCapacity)?;
        if self.closing || end > self.layout.total_bytes {
            return Err(if self.closing { Mesh3dFault::Closing } else { Mesh3dFault::ByteCapacity });
        }
        let mut result = [0; N];
        for (delta, output) in result.iter_mut().enumerate() {
            let at = absolute + delta;
            let page = self.pages.as_ref().ok_or(Mesh3dFault::Incomplete)?[at / MESH3D_PAGE_BYTES].as_ref().ok_or(Mesh3dFault::Incomplete)?;
            *output = page.bytes[at % MESH3D_PAGE_BYTES];
        }
        Ok(result)
    }

    fn item_bytes<const N: usize>(&self, field: Mesh3dField, item: u32) -> Result<[u8; N], Mesh3dFault> {
        if N != Mesh3dSchema::field_item_bytes(field) || item >= self.schema.field_items(field) {
            return Err(Mesh3dFault::Schema);
        }
        let absolute = self.layout.offsets[field.index()].checked_add(item as usize * N).ok_or(Mesh3dFault::ByteCapacity)?;
        self.read(absolute)
    }

    fn terminal_is_complete(&self) -> bool {
        self.component_complete && self.component_fault.is_none() && self.allocated_pages == self.layout.page_count
            && self.written.iter().enumerate().all(|(index, count)| {
                *count
                    == self.schema.field_items([Mesh3dField::Positions, Mesh3dField::Normals, Mesh3dField::Indices, Mesh3dField::FaceIds, Mesh3dField::VertexIds, Mesh3dField::Edges, Mesh3dField::EdgeIds, Mesh3dField::Uvs, Mesh3dField::Colors,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive,Mesh3dField::Tangents][index])
            })
    }

    fn close_demands(&self,work:usize)->Result<RetirementDemand,Mesh3dFault> {
        if self.close_page<self.allocated_pages{return Ok(RetirementDemand{release_bytes:MESH3D_PAGE_BYTES,depth:1,..Default::default()});}
        if let Some(owner)=&self.component_retirement{return controlled_demands(owner,work);}
        if self.component_references.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=&self.scratch_retirement{return controlled_demands(owner,work);}
        if self.component_seen.owns_backing()||self.component_ineligible.owns_backing(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        Ok(RetirementDemand{release_bytes:if self.pages.is_some(){mesh3d_begin_capacity_byte_demand()}else{0},depth:1,..Default::default()})
    }

    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<Mesh3dOwnershipStep,Mesh3dFault> {
        self.closing=true;
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(Mesh3dOwnershipStep::default());}
        if self.close_page<self.allocated_pages {
            if grant.maximum_release_bytes<MESH3D_PAGE_BYTES{return Ok(Mesh3dOwnershipStep::default());}
            self.pages.as_mut().unwrap()[usize::from(self.close_page)]=None;self.close_page+=1;
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,released_bytes:MESH3D_PAGE_BYTES,..Default::default()},..Default::default()});
        }
        if let Some(owner)=self.component_retirement.as_mut() {
            let step=owner.step(grant).map_err(|_|Mesh3dFault::Closing)?;let progress=step.progress();
            if owner.terminal_is_empty(){self.component_retirement=None;}
            return Ok(Mesh3dOwnershipStep{progress,..Default::default()});
        }
        if let Some(table)=self.component_references.take() {
            match ControlledRetirement::new(table){Ok(owner)=>self.component_retirement=Some(owner),Err((_,table))=>{self.component_references=Some(table);return Err(Mesh3dFault::Closing);}}
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});
        }
        if let Some(owner)=self.scratch_retirement.as_mut() {
            let step=owner.step(grant).map_err(|_|Mesh3dFault::Closing)?;let progress=step.progress();
            if owner.terminal_is_empty(){self.scratch_retirement=None;}
            return Ok(Mesh3dOwnershipStep{progress,..Default::default()});
        }
        if self.component_seen.owns_backing()||self.component_ineligible.owns_backing() {
            let original=(std::mem::take(&mut self.component_seen),std::mem::take(&mut self.component_ineligible));
            self.scratch_retirement=Some(ControlledRetirement::new(original).map_err(|_|Mesh3dFault::Closing)?);
            self.component_probe=None;
            return Ok(Mesh3dOwnershipStep{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});
        }
        let release=if self.pages.is_some(){mesh3d_begin_capacity_byte_demand()}else{0};
        if grant.maximum_release_bytes<release{return Ok(Mesh3dOwnershipStep::default());}
        self.pages=None;self.layout.total_bytes=0;self.layout.page_count=0;self.allocated_pages=0;self.written=[0;14];
        Ok(Mesh3dOwnershipStep{complete:true,progress:RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()}})
    }

}

enum Mesh3dSlotState {
    Writing(Mesh3dOwner),
    Ready(Mesh3dOwner),
    Closing(Mesh3dOwner),
    Transition,
}

impl Drop for Mesh3dOwner {
    fn drop(&mut self) {assert!(std::thread::panicking()||(self.pages.is_none()&&self.component_references.is_none()&&self.component_retirement.is_none()&&self.scratch_retirement.is_none()&&!self.component_seen.owns_backing()&&!self.component_ineligible.owns_backing()),"mesh owner dropped before admitted terminal retirement");}
}

struct Mesh3dSlot {
    epoch: u64,
    reserved_pages: u16,
    state: Mesh3dSlotState,
}

struct Mesh3dAuthority {
    slots: [Option<Mesh3dSlot>; MESH3D_AUTHORITY_CAPACITY],
    epochs: [u64; MESH3D_AUTHORITY_CAPACITY],
    reserved_pages: usize,
}

impl Mesh3dAuthority {
    const fn new() -> Self {
        Self { slots: [const {None};MESH3D_AUTHORITY_CAPACITY], epochs: [0; MESH3D_AUTHORITY_CAPACITY], reserved_pages: 0 }
    }

    fn begin(&mut self, generation: u64, revision: u64, schema: Mesh3dSchema,grant:RetainedCloneGrant) -> Result<(Mesh3dWriteToken,RetainedCloneProgress), Mesh3dFault> {
        let layout = schema.layout()?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<mesh3d_begin_capacity_byte_demand(){return Err(Mesh3dFault::ByteCapacity);}
        let pages = usize::from(layout.page_count);
        if self.reserved_pages.checked_add(pages).is_none_or(|pages| pages > MESH3D_AUTHORITY_PAGE_CAPACITY) {
            return Err(Mesh3dFault::PageCapacity);
        }
        let slot = self.slots.iter().position(Option::is_none).ok_or(Mesh3dFault::ItemCapacity)?;
        let epoch = self.epochs[slot].wrapping_add(1).max(1);
        self.epochs[slot] = epoch;
        self.slots[slot] = Some(Mesh3dSlot { epoch, reserved_pages: layout.page_count, state: Mesh3dSlotState::Writing(Mesh3dOwner::new(generation, revision, schema, layout)) });
        self.reserved_pages += pages;
        Ok((Mesh3dWriteToken { slot: slot as u16, epoch, generation, revision },RetainedCloneProgress{copied_items:1,retained_capacity_bytes:mesh3d_begin_capacity_byte_demand(),..Default::default()}))
    }

    fn writing(&mut self, token: Mesh3dWriteToken) -> Result<&mut Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        match &mut slot.state {
            Mesh3dSlotState::Writing(owner) if owner.generation == token.generation && owner.revision == token.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    fn writing_ref(&self, token: Mesh3dWriteToken) -> Result<&Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get(usize::from(token.slot)).and_then(Option::as_ref).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        match &slot.state {
            Mesh3dSlotState::Writing(owner) if owner.generation == token.generation && owner.revision == token.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    fn ready(&self, lease: Mesh3dLease) -> Result<&Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        match &slot.state {
            Mesh3dSlotState::Ready(owner) if owner.generation == lease.generation && owner.revision == lease.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    fn ready_mut(&mut self, lease: Mesh3dLease) -> Result<&mut Mesh3dOwner, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        match &mut slot.state {
            Mesh3dSlotState::Ready(owner) if owner.generation == lease.generation && owner.revision == lease.revision => Ok(owner),
            Mesh3dSlotState::Closing(_) => Err(Mesh3dFault::Closing),
            _ => Err(Mesh3dFault::Stale),
        }
    }

    fn reserve_component_credit(&mut self, token: Mesh3dWriteToken, credit: usize) -> Result<(), Mesh3dFault> {
        let geometry_pages = usize::from(self.writing_ref(token)?.layout.page_count);
        let bytes = geometry_pages.checked_mul(MESH3D_PAGE_BYTES).and_then(|bytes| bytes.checked_add(credit)).ok_or(Mesh3dFault::ByteCapacity)?;
        if bytes > MESH3D_OWNER_BYTE_CAPACITY { return Err(Mesh3dFault::ByteCapacity); }
        let pages = bytes.div_ceil(MESH3D_PAGE_BYTES);
        let previous = usize::from(self.slots[usize::from(token.slot)].as_ref().ok_or(Mesh3dFault::Stale)?.reserved_pages);
        let added = pages.saturating_sub(previous);
        if self.reserved_pages.checked_add(added).is_none_or(|pages| pages > MESH3D_AUTHORITY_PAGE_CAPACITY) { return Err(Mesh3dFault::PageCapacity); }
        self.reserved_pages += added;
        self.slots[usize::from(token.slot)].as_mut().ok_or(Mesh3dFault::Stale)?.reserved_pages = pages as u16;
        self.writing(token)?.component_credit = credit;
        Ok(())
    }

    fn move_component_references(&mut self, token: Mesh3dWriteToken, original: &mut ComponentReferenceTable,grant:RetainedCloneGrant) -> Result<RetainedCloneProgress, Mesh3dFault> {
        let owner = self.writing(token)?;
        if owner.component_references.is_some() { return Err(Mesh3dFault::Order); }
        let lengths=component_lengths(original)?;let birth=mesh3d_component_capacity_byte_demand(original)?;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<birth{return Err(Mesh3dFault::ByteCapacity);}
        owner.component_seen=NumericSeen::new(*lengths.iter().max().unwrap());owner.component_ineligible=IneligibleGroups::new(lengths);
        owner.component_references = Some(std::mem::take(original));
        owner.component_complete = false;
        let result = owner.component_header_credit().and_then(|credit| self.reserve_component_credit(token, credit));
        if let Err(fault) = result { self.writing(token)?.component_fault = Some(fault); }
        result.map(|()|RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()})
    }

    fn component_admission_step(&mut self, token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dComponentAdmissionStep, Mesh3dFault> {
        let maximum_items=grant.maximum_items;let maximum_bytes=grant.maximum_copy_bytes;
        let owner = self.writing_ref(token)?;
        if let Some(fault) = owner.component_fault { return Err(fault); }
        if owner.component_complete || maximum_items == 0 || maximum_bytes == 0||grant.maximum_depth==0 { return Ok(Mesh3dComponentAdmissionStep { complete: owner.component_complete, ..Default::default() }); }
        let bytes = owner.component_turn_byte_demand();
        if maximum_bytes < bytes { return Ok(Mesh3dComponentAdmissionStep::default()); }
        let (credit, number) = owner.component_turn_credit()?;
        if let Err(fault) = self.reserve_component_credit(token, credit) { self.writing(token)?.component_fault = Some(fault); return Err(fault); }
        let mut step=self.writing(token)?.component_turn(number);step.ownership=RetainedCloneProgress{copied_items:step.processed_items+step.retired_items,copied_bytes:step.processed_bytes+step.retired_bytes,..Default::default()};Ok(step)
    }

    fn seal(&mut self, token: Mesh3dWriteToken) -> Result<Mesh3dLease, Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Writing(owner) = &slot.state else { return Err(Mesh3dFault::Stale) };
        if let Some(fault) = owner.component_fault { return Err(fault); }
        if !owner.terminal_is_complete() {
            return Err(Mesh3dFault::Incomplete);
        }
        if owner.schema.indices == 0 && owner.schema.edges == 0 && !owner.has_topological_vertex { return Err(Mesh3dFault::Schema); }
        let owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Writing(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        slot.state = Mesh3dSlotState::Ready(owner);
        Ok(Mesh3dLease { slot: token.slot, epoch: token.epoch, generation: token.generation, revision: token.revision })
    }

    fn begin_close_write(&mut self, token: Mesh3dWriteToken) -> Result<(), Mesh3dFault> {
        let slot = self.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == token.epoch).ok_or(Mesh3dFault::Stale)?;
        if matches!(slot.state, Mesh3dSlotState::Closing(_)) {
            return Ok(());
        }
        let mut owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Writing(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        owner.closing = true;
        slot.state = Mesh3dSlotState::Closing(owner);
        Ok(())
    }

    fn begin_close(&mut self, lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
        if self.ready(lease).is_ok_and(|owner| owner.resource_claimed || owner.resource_loans != 0) { return Err(Mesh3dFault::Order); }
        let slot = self.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).filter(|slot| slot.epoch == lease.epoch).ok_or(Mesh3dFault::Stale)?;
        if matches!(slot.state, Mesh3dSlotState::Closing(_)) {
            return Ok(());
        }
        let mut owner = match std::mem::replace(&mut slot.state, Mesh3dSlotState::Transition) {
            Mesh3dSlotState::Ready(owner) => owner,
            other => {
                slot.state = other;
                return Err(Mesh3dFault::Stale);
            }
        };
        owner.closing = true;
        slot.state = Mesh3dSlotState::Closing(owner);
        Ok(())
    }

    fn close_demands(&self,slot_index:u16,epoch:u64,work:usize)->Result<RetirementDemand,Mesh3dFault> {
        let slot=self.slots.get(usize::from(slot_index)).and_then(Option::as_ref).filter(|slot|slot.epoch==epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Closing(owner)=&slot.state else{return Err(Mesh3dFault::Closing)};owner.close_demands(work)
    }

    fn begin_close_resource(&mut self, lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
        let owner = self.ready_mut(lease)?;
        if !owner.resource_claimed || !owner.resource_closing || owner.resource_loans != 0 { return Err(Mesh3dFault::Order); }
        owner.resource_claimed = false;
        self.begin_close(lease)
    }

    fn close_step(&mut self, slot_index: u16, epoch: u64,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
        let index = usize::from(slot_index);
        let slot = self.slots.get_mut(index).and_then(Option::as_mut).filter(|slot| slot.epoch == epoch).ok_or(Mesh3dFault::Stale)?;
        let Mesh3dSlotState::Closing(owner) = &mut slot.state else { return Err(Mesh3dFault::Closing) };
        let step=owner.close_step(grant)?;
        if !step.complete {
            return Ok(step);
        }
        let pages = usize::from(slot.reserved_pages);
        let _ = owner;
        self.slots[index] = None;
        self.reserved_pages = self.reserved_pages.saturating_sub(pages);
        Ok(step)
    }
}

fn mesh3d_authority() -> &'static semio_framework_value::retirement::controlled::RetainedOwnerGate<Mesh3dAuthority> {
    static AUTHORITY:semio_framework_value::retirement::controlled::RetainedOwnerGate<Mesh3dAuthority>=semio_framework_value::retirement::controlled::RetainedOwnerGate::new(Mesh3dAuthority::new());
    &AUTHORITY
}

pub const fn mesh3d_begin_capacity_byte_demand()->usize {size_of::<[Option<Mesh3dPage>;MESH3D_OWNER_PAGE_CAPACITY]>()}

pub fn mesh3d_begin(generation: u64, revision: u64, schema: Mesh3dSchema,grant:RetainedCloneGrant) -> Result<(Mesh3dWriteToken,RetainedCloneProgress), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin(generation, revision, schema,grant)
}

pub fn mesh3d_allocate_capacity_byte_demand(token:Mesh3dWriteToken)->Result<usize,Mesh3dFault> {
    let authority=mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?;let owner=authority.writing_ref(token)?;Ok(if owner.allocated_pages==owner.layout.page_count{0}else{MESH3D_PAGE_BYTES})
}

pub fn mesh3d_allocate_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    Ok(mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.allocate_step(grant))
}

pub fn mesh3d_write_vec3(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 3]) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 12];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    let item = owner.write(field, &bytes)?;
    if field == Mesh3dField::Positions {
        for (axis, coordinate) in value.into_iter().enumerate() {
            owner.aabb_min[axis] = owner.aabb_min[axis].min(coordinate);
            owner.aabb_max[axis] = owner.aabb_max[axis].max(coordinate);
        }
    }
    let _ = item;
    Ok(())
}

pub fn mesh3d_write_edge(token: Mesh3dWriteToken, value: [[f32; 3]; 2]) -> Result<(), Mesh3dFault> {
    if !value.iter().flatten().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 24];
    for (index, value) in value.into_iter().flatten().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    owner.write(Mesh3dField::Edges, &bytes)?;
    for point in value {
        for (axis, coordinate) in point.into_iter().enumerate() {
            owner.aabb_min[axis] = owner.aabb_min[axis].min(coordinate);
            owner.aabb_max[axis] = owner.aabb_max[axis].max(coordinate);
        }
    }
    Ok(())
}

pub fn mesh3d_write_vec2(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 2]) -> Result<(), Mesh3dFault> {
    if !matches!(field,Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 8];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &bytes)?;
    Ok(())
}

pub fn mesh3d_write_vec4(token: Mesh3dWriteToken, field: Mesh3dField, value: [f32; 4]) -> Result<(), Mesh3dFault> {
    if !matches!(field,Mesh3dField::Colors | Mesh3dField::Tangents) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut bytes = [0; 16];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &bytes)?;
    Ok(())
}

pub fn mesh3d_write_u32(token: Mesh3dWriteToken, field: Mesh3dField, value: u32) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
        return Err(Mesh3dFault::Schema);
    }
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.writing(token)?.write(field, &value.to_le_bytes())?;
    Ok(())
}

pub fn mesh3d_read_write_vec3(token: Mesh3dWriteToken, field: Mesh3dField, item: u32) -> Result<[f32; 3], Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) {
        return Err(Mesh3dFault::Schema);
    }
    let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing_ref(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    let bytes = owner.item_bytes::<12>(field, item)?;
    Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
}

pub fn mesh3d_read_write_u32(token: Mesh3dWriteToken, field: Mesh3dField, item: u32) -> Result<u32, Mesh3dFault> {
    if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
        return Err(Mesh3dFault::Schema);
    }
    let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing_ref(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    Ok(u32::from_le_bytes(owner.item_bytes::<4>(field, item)?))
}

pub fn mesh3d_update_vec3(token: Mesh3dWriteToken, field: Mesh3dField, item: u32, value: [f32; 3]) -> Result<(), Mesh3dFault> {
    if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) || !value.iter().all(|value| value.is_finite()) {
        return Err(Mesh3dFault::Schema);
    }
    let mut authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
    let owner = authority.writing(token)?;
    if item >= owner.written[field.index()] {
        return Err(Mesh3dFault::Incomplete);
    }
    let absolute = owner.layout.offsets[field.index()].checked_add(item as usize * 12).ok_or(Mesh3dFault::ByteCapacity)?;
    let mut bytes = [0; 12];
    for (index, value) in value.into_iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    owner.write_at(absolute, &bytes)
}

pub fn mesh3d_seal(token: Mesh3dWriteToken) -> Result<Mesh3dLease, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.seal(token)
}

/// 🎯️ Moves the original label table into the same mesh owner before publication.
pub fn mesh3d_component_capacity_byte_demand(original:&ComponentReferenceTable)->Result<usize,Mesh3dFault> {
    let lengths=component_lengths(original)?;NumericSeen::capacity_bytes(*lengths.iter().max().unwrap()).and_then(|bytes|bytes.checked_add(lengths.iter().sum())).ok_or(Mesh3dFault::ByteCapacity)
}

pub fn mesh3d_move_component_references(token: Mesh3dWriteToken, original: &mut ComponentReferenceTable,grant:RetainedCloneGrant) -> Result<RetainedCloneProgress, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.move_component_references(token, original,grant)
}

/// 🧮️ Admits one original typed label or retires one scratch entry under fixed credits.
pub fn mesh3d_component_admission_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dComponentAdmissionStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.component_admission_step(token,grant)
}

pub fn mesh3d_abort(token: Mesh3dWriteToken) -> Result<(), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin_close_write(token)
}

pub fn mesh3d_abort_demands(token:Mesh3dWriteToken,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?.close_demands(token.slot,token.epoch,work)
}

pub fn mesh3d_abort_step(token: Mesh3dWriteToken,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.close_step(token.slot, token.epoch,grant)
}

pub fn mesh3d_begin_close(lease: Mesh3dLease) -> Result<(), Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.begin_close(lease)
}

pub fn mesh3d_close_demands(lease:Mesh3dLease,work:usize)->Result<RetirementDemand,Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_|Mesh3dFault::Busy)?.close_demands(lease.slot,lease.epoch,work)
}

pub fn mesh3d_close_step(lease: Mesh3dLease,grant:RetainedCloneGrant) -> Result<Mesh3dOwnershipStep, Mesh3dFault> {
    mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.close_step(lease.slot, lease.epoch,grant)
}

pub fn mesh3d_terminal_is_empty(lease: Mesh3dLease) -> bool {
    mesh3d_authority().try_lock().is_ok_and(|authority| authority.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).is_none_or(|slot| slot.epoch != lease.epoch))
}

#[path = "../📦️resource/🦀️.rs"]
mod mesh3d_resource;
pub use mesh3d_resource::{Mesh3dResourceIdentity, Mesh3dResourceLoan, Mesh3dResourceOwner, Mesh3dResourceReceiving, Mesh3dReceivingStep, MESH3D_RESOURCE_COMPONENT_COUNT_CAPACITY};

impl Mesh3dLease {
    /// 📃️ Lends one original initialized page while the existing native arena guard is held.
    pub fn with_original_page<R>(self, index: usize, read: impl FnOnce(&[u8]) -> R) -> Result<R, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        if index >= usize::from(owner.layout.page_count) { return Err(Mesh3dFault::Schema); }
        let page = owner.pages.as_ref().and_then(|pages| pages.get(index)).and_then(Option::as_ref).ok_or(Mesh3dFault::Incomplete)?;
        let initialized = owner.layout.total_bytes.saturating_sub(index * MESH3D_PAGE_BYTES).min(MESH3D_PAGE_BYTES);
        Ok(read(&page.bytes[..initialized]))
    }

    /// 📏️ Reads the original page census without allocating a transport mirror.
    pub fn original_page_count(self) -> Result<usize, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        Ok(usize::from(authority.ready(self)?.layout.page_count))
    }

    /// 📐️ Reads the initialized geometry extent of the original arena owner.
    pub fn original_geometry_bytes(self) -> Result<usize, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        Ok(authority.ready(self)?.layout.total_bytes)
    }

    /// 🧮️ Reads the three original topological label counts under the arena guard.
    pub fn original_component_counts(self) -> Result<[usize; 3], Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        Ok(std::array::from_fn(|domain| owner.component_references.as_ref().and_then(|table| table.get(["face", "edge", "vertex"][domain])).map_or(0, Vec::len)))
    }

    /// 🏷️ Lends one complete original label, including labels ineligible for numeric picking.
    pub fn with_original_component_label<R>(self, domain: usize, group: usize, read: impl FnOnce(&str) -> R) -> Result<R, Mesh3dFault> {
        let kind = ["face", "edge", "vertex"].get(domain).ok_or(Mesh3dFault::Schema)?;
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let label = owner.component_references.as_ref().and_then(|table| table.get(*kind)).and_then(|labels| labels.get(group)).ok_or(Mesh3dFault::Schema)?;
        Ok(read(label.as_str()))
    }

    /// 🔖️ Reads one original canonical label without a floating point conversion.
    pub fn component_label(self, field: Mesh3dField, group: u32) -> Result<Option<u64>, Mesh3dFault> {
        let (kind, domain) = match field { Mesh3dField::FaceIds => ("face", 0u64), Mesh3dField::EdgeIds => ("edge", 1u64), Mesh3dField::VertexIds => ("vertex", 2u64), _ => return Err(Mesh3dFault::Schema) };
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let Some(label) = owner.component_references.as_ref().and_then(|map| map.get(kind)).and_then(|labels| labels.get(group as usize)) else { return Ok(None); };
        if owner.component_ineligible.contains(&((domain << 32) | u64::from(group))) { return Err(Mesh3dFault::Schema); }
        mesh3d_component_number(label).map(Some).ok_or(Mesh3dFault::Schema)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }

    pub fn revision(self) -> u64 {
        self.revision
    }

    pub fn schema(self) -> Result<Mesh3dSchema, Mesh3dFault> {
        Ok(mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.schema)
    }

    pub fn aabb(self) -> Result<([f32; 3], [f32; 3]), Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        Ok((owner.aabb_min, owner.aabb_max))
    }

    pub fn cursor(self, field: Mesh3dField) -> Result<Mesh3dItemCursor, Mesh3dFault> {
        let len = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.schema.field_items(field);
        Ok(Mesh3dItemCursor { lease: self, field, index: 0, len })
    }

    pub fn page_cursor(self, field: Mesh3dField) -> Result<Mesh3dPageCursor, Mesh3dFault> {
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self)?;
        let start = owner.layout.offsets[field.index()];
        let bytes = usize::try_from(owner.schema.field_items(field)).ok().and_then(|items| items.checked_mul(Mesh3dSchema::field_item_bytes(field))).ok_or(Mesh3dFault::ByteCapacity)?;
        Ok(Mesh3dPageCursor { lease: self, absolute: start, end: start + bytes })
    }

    pub fn vec2(self, field: Mesh3dField, item: u32) -> Result<[f32; 2], Mesh3dFault> {
        if !matches!(field,Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<8>(field, item)?;
        Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..].try_into().expect("fixed mesh scalar"))])
    }

    pub fn vec3(self, field: Mesh3dField, item: u32) -> Result<[f32; 3], Mesh3dFault> {
        if !matches!(field, Mesh3dField::Positions | Mesh3dField::Normals) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<12>(field, item)?;
        Ok([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
    }

    pub fn vec4(self, field: Mesh3dField, item: u32) -> Result<[f32; 4], Mesh3dFault> {
        if !matches!(field,Mesh3dField::Colors | Mesh3dField::Tangents) {
            return Err(Mesh3dFault::Schema);
        }
        let bytes = self.read::<16>(field, item)?;
        Ok(std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"))))
    }

    pub fn edge(self, item: u32) -> Result<[[f32; 3]; 2], Mesh3dFault> {
        let bytes = self.read::<24>(Mesh3dField::Edges, item)?;
        let values: [f32; 6] = std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar")));
        Ok([[values[0], values[1], values[2]], [values[3], values[4], values[5]]])
    }

    pub fn u32(self, field: Mesh3dField, item: u32) -> Result<u32, Mesh3dFault> {
        if !matches!(field, Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds) {
            return Err(Mesh3dFault::Schema);
        }
        Ok(u32::from_le_bytes(self.read::<4>(field, item)?))
    }

    fn read<const N: usize>(self, field: Mesh3dField, item: u32) -> Result<[u8; N], Mesh3dFault> {
        mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?.ready(self)?.item_bytes(field, item)
    }
}

pub struct Mesh3dItemCursor {
    lease: Mesh3dLease,
    field: Mesh3dField,
    index: u32,
    len: u32,
}

pub struct Mesh3dPageCursor {
    lease: Mesh3dLease,
    absolute: usize,
    end: usize,
}

impl Mesh3dPageCursor {
    pub fn next<R>(&mut self, read: impl FnOnce(&[u8]) -> R) -> Result<Option<R>, Mesh3dFault> {
        if self.absolute == self.end {
            return Ok(None);
        }
        let authority = mesh3d_authority().try_lock().map_err(|_| Mesh3dFault::Busy)?;
        let owner = authority.ready(self.lease)?;
        let page_index = self.absolute / MESH3D_PAGE_BYTES;
        let page_offset = self.absolute % MESH3D_PAGE_BYTES;
        let page = owner.pages.as_ref().and_then(|pages|pages.get(page_index)).and_then(Option::as_ref).ok_or(Mesh3dFault::Incomplete)?;
        let count = (MESH3D_PAGE_BYTES - page_offset).min(self.end - self.absolute);
        let result = read(&page.bytes[page_offset..page_offset + count]);
        self.absolute += count;
        Ok(Some(result))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mesh3dItem {
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Edge([[f32; 3]; 2]),
    U32(u32),
}

impl Mesh3dItemCursor {
    pub fn read_next(&mut self) -> Result<Option<Mesh3dItem>, Mesh3dFault> {
        if self.index == self.len {
            return Ok(None);
        }
        let bytes = match self.field {
            Mesh3dField::Positions | Mesh3dField::Normals => {
                let bytes = self.lease.read::<12>(self.field, self.index)?;
                Mesh3dItem::Vec3([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..8].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[8..].try_into().expect("fixed mesh scalar"))])
            }
            Mesh3dField::Edges => {
                let bytes = self.lease.read::<24>(self.field, self.index)?;
                let mut values = [0.0; 6];
                for (index, value) in values.iter_mut().enumerate() {
                    *value = f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"));
                }
                Mesh3dItem::Edge([[values[0], values[1], values[2]], [values[3], values[4], values[5]]])
            }
            Mesh3dField::Uvs | Mesh3dField::UvsMetallicRoughness | Mesh3dField::UvsNormal | Mesh3dField::UvsOcclusion | Mesh3dField::UvsEmissive => {
                let bytes = self.lease.read::<8>(self.field, self.index)?;
                Mesh3dItem::Vec2([f32::from_le_bytes(bytes[..4].try_into().expect("fixed mesh scalar")), f32::from_le_bytes(bytes[4..].try_into().expect("fixed mesh scalar"))])
            }
            Mesh3dField::Colors | Mesh3dField::Tangents => {
                let bytes = self.lease.read::<16>(self.field, self.index)?;
                Mesh3dItem::Vec4(std::array::from_fn(|index| f32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().expect("fixed mesh scalar"))))
            }
            Mesh3dField::Indices | Mesh3dField::FaceIds | Mesh3dField::VertexIds | Mesh3dField::EdgeIds => Mesh3dItem::U32(u32::from_le_bytes(self.lease.read::<4>(self.field, self.index)?)),
        };
        self.index += 1;
        Ok(Some(bytes))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SceneColorSource3d {
    SemanticNeutral,
    #[default]
    Authored,
    Environment,
}

impl SceneColorSource3d {
    pub const fn snapshot_flag(self) -> u16 {
        self as u16
    }

    pub const fn from_snapshot_flag(flag: u16) -> Self {
        match flag {
            0 => Self::SemanticNeutral,
            2 => Self::Environment,
            _ => Self::Authored,
        }
    }
}

/// ✂️ How one instance takes part in the pass's section plane ([`ScenePass3d::section_clip`]). `Clipped` fragments on the removed side of the plane are discarded;
/// `StencilToggle` is the cap's parity pass over a clipped solid (it writes only the stencil's section bit, never colour or depth);
/// `Cap` is the flat plane that is drawn where that bit is set and the cut therefore exposes a solid's inside.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SceneSectionRole3d {
    #[default]
    None,
    Clipped,
    StencilToggle,
    Cap,
}

impl SceneSectionRole3d {
    /// ✂️ Whether the instance's fragments are tested against the section plane.
    pub fn clips(self) -> bool {
        matches!(self, Self::Clipped | Self::StencilToggle)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneInstanceMaterial3d {
    pub color_source: SceneColorSource3d,
    pub preserve_vertex_color: bool,
    pub emissive_intensity: f32,
    pub metalness: f32,
    pub roughness: f32,
    pub section: SceneSectionRole3d,
}

impl Default for SceneInstanceMaterial3d {
    fn default() -> Self {
        Self { color_source: SceneColorSource3d::default(), preserve_vertex_color: false, emissive_intensity: 0.0, metalness: 0.0, roughness: 1.0, section: SceneSectionRole3d::None }
    }
}

#[derive(Clone, Debug)]
pub struct Instance3d {
    pub component_source: Option<ComponentSource3d>,
    pub id: String,
    pub model: Mat4,
    pub color: [f32; 4],
    pub selected: bool,
    pub hovered: bool,
    pub material: SceneInstanceMaterial3d,
}

/// 🧬️ The exact analytic source identity carried by one scene instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentSource3d {
    handle: [u8; 64],
    revision: [u8; 64],
}

impl ComponentSource3d {
    pub fn new(handle: &str, revision: &str) -> Option<Self> {
        let valid = |value: &str| value.len() == 64 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
        if !valid(handle) || !valid(revision) { return None; }
        Some(Self { handle: handle.as_bytes().try_into().ok()?, revision: revision.as_bytes().try_into().ok()? })
    }

    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("validated component source handle") }

    pub fn revision(&self) -> &str { std::str::from_utf8(&self.revision).expect("validated component source revision") }
}

impl Instance3d {
    pub fn model_from_trs(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Mat4 {
        mat4_translation_m(vec3_from_array_m(position)).mul_m(mat4_from_quat_m(rotation[0], rotation[1], rotation[2], rotation[3])).mul_m(mat4_scale_vec_m(vec3_from_array_m(scale)))
    }
}
//#endregion Mesh

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️tests/🧰️mesh-testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️mesh-unit/🦀️.rs"]
mod tests;

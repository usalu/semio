//! 🏢️ Sparse patches of the project, wall sweeps, site, buildings, storeys, grid lines, spaces, zones and area schemes.

use super::*;

/// 🩹 Sparse patch of [`WallSweep`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct WallSweepPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub side: Option<WallSide>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub inset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<WallSweep> for WallSweepPatch {
    fn write(&self, base: &WallSweep) -> WallSweep {
        WallSweep { host: take(&self.host, &base.host), side: take(&self.side, &base.side), profile: take(&self.profile, &base.profile), height: take(&self.height, &base.height), inset: take(&self.inset, &base.inset), material: take(&self.material, &base.material), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &WallSweep) -> Self {
        Self { host: restore(&self.host, &base.host), side: restore(&self.side, &base.side), profile: restore(&self.profile, &base.profile), height: restore(&self.height, &base.height), inset: restore(&self.inset, &base.inset), material: restore(&self.material, &base.material), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.host, later.host);
        merge_slot(&mut self.side, later.side);
        merge_slot(&mut self.profile, later.profile);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.inset, later.inset);
        merge_slot(&mut self.material, later.material);
        merge_slot(&mut self.name, later.name);
    }
    fn minimal(&self, base: &WallSweep) -> Self {
        Self { host: changed(&self.host, &base.host), side: changed(&self.side, &base.side), profile: changed(&self.profile, &base.profile), height: changed(&self.height, &base.height), inset: changed(&self.inset, &base.inset), material: changed(&self.material, &base.material), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.host.is_some() {
            out.push("host".to_string());
        }
        if self.side.is_some() {
            out.push("side".to_string());
        }
        if self.profile.is_some() {
            out.push("profile".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.inset.is_some() {
            out.push("inset".to_string());
        }
        if self.material.is_some() {
            out.push("material".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Project`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ProjectPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase_names: Option<Vec<String>>,
}

impl Patch<Project> for ProjectPatch {
    fn write(&self, base: &Project) -> Project {
        Project { name: take(&self.name, &base.name), description: take(&self.description, &base.description), author: take(&self.author, &base.author), organization: take(&self.organization, &base.organization), phase_names: take(&self.phase_names, &base.phase_names) }
    }
    fn restoring(&self, base: &Project) -> Self {
        Self { name: restore(&self.name, &base.name), description: restore(&self.description, &base.description), author: restore(&self.author, &base.author), organization: restore(&self.organization, &base.organization), phase_names: restore(&self.phase_names, &base.phase_names) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.description, later.description);
        merge_slot(&mut self.author, later.author);
        merge_slot(&mut self.organization, later.organization);
        merge_slot(&mut self.phase_names, later.phase_names);
    }
    fn minimal(&self, base: &Project) -> Self {
        Self { name: changed(&self.name, &base.name), description: changed(&self.description, &base.description), author: changed(&self.author, &base.author), organization: changed(&self.organization, &base.organization), phase_names: changed(&self.phase_names, &base.phase_names) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.description.is_some() {
            out.push("description".to_string());
        }
        if self.author.is_some() {
            out.push("author".to_string());
        }
        if self.organization.is_some() {
            out.push("organization".to_string());
        }
        if self.phase_names.is_some() {
            out.push("phase_names".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Site`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SitePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub true_north: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<Vec<Point2>>,
}

impl Patch<Site> for SitePatch {
    fn write(&self, base: &Site) -> Site {
        Site { name: take(&self.name, &base.name), latitude: take(&self.latitude, &base.latitude), longitude: take(&self.longitude, &base.longitude), elevation: take(&self.elevation, &base.elevation), true_north: take(&self.true_north, &base.true_north), boundary: take(&self.boundary, &base.boundary) }
    }
    fn restoring(&self, base: &Site) -> Self {
        Self { name: restore(&self.name, &base.name), latitude: restore(&self.latitude, &base.latitude), longitude: restore(&self.longitude, &base.longitude), elevation: restore(&self.elevation, &base.elevation), true_north: restore(&self.true_north, &base.true_north), boundary: restore(&self.boundary, &base.boundary) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.latitude, later.latitude);
        merge_slot(&mut self.longitude, later.longitude);
        merge_slot(&mut self.elevation, later.elevation);
        merge_slot(&mut self.true_north, later.true_north);
        merge_slot(&mut self.boundary, later.boundary);
    }
    fn minimal(&self, base: &Site) -> Self {
        Self { name: changed(&self.name, &base.name), latitude: changed(&self.latitude, &base.latitude), longitude: changed(&self.longitude, &base.longitude), elevation: changed(&self.elevation, &base.elevation), true_north: changed(&self.true_north, &base.true_north), boundary: changed(&self.boundary, &base.boundary) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.latitude.is_some() {
            out.push("latitude".to_string());
        }
        if self.longitude.is_some() {
            out.push("longitude".to_string());
        }
        if self.elevation.is_some() {
            out.push("elevation".to_string());
        }
        if self.true_north.is_some() {
            out.push("true_north".to_string());
        }
        if self.boundary.is_some() {
            out.push("boundary".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Building`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct BuildingPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,
}

impl Patch<Building> for BuildingPatch {
    fn write(&self, base: &Building) -> Building {
        Building { site: take(&self.site, &base.site), name: take(&self.name, &base.name), origin: take(&self.origin, &base.origin), rotation: take(&self.rotation, &base.rotation), elevation: take(&self.elevation, &base.elevation) }
    }
    fn restoring(&self, base: &Building) -> Self {
        Self { site: restore(&self.site, &base.site), name: restore(&self.name, &base.name), origin: restore(&self.origin, &base.origin), rotation: restore(&self.rotation, &base.rotation), elevation: restore(&self.elevation, &base.elevation) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.site, later.site);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.origin, later.origin);
        merge_slot(&mut self.rotation, later.rotation);
        merge_slot(&mut self.elevation, later.elevation);
    }
    fn minimal(&self, base: &Building) -> Self {
        Self { site: changed(&self.site, &base.site), name: changed(&self.name, &base.name), origin: changed(&self.origin, &base.origin), rotation: changed(&self.rotation, &base.rotation), elevation: changed(&self.elevation, &base.elevation) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.site.is_some() {
            out.push("site".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.origin.is_some() {
            out.push("origin".to_string());
        }
        if self.rotation.is_some() {
            out.push("rotation".to_string());
        }
        if self.elevation.is_some() {
            out.push("elevation".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Storey`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct StoreyPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub cut_height: Option<Assigned<Option<f64>>>,
}

impl Patch<Storey> for StoreyPatch {
    fn write(&self, base: &Storey) -> Storey {
        Storey { building: take(&self.building, &base.building), name: take(&self.name, &base.name), level: take(&self.level, &base.level), height: take(&self.height, &base.height), cut_height: take_assigned(&self.cut_height, &base.cut_height) }
    }
    fn restoring(&self, base: &Storey) -> Self {
        Self { building: restore(&self.building, &base.building), name: restore(&self.name, &base.name), level: restore(&self.level, &base.level), height: restore(&self.height, &base.height), cut_height: restore_assigned(&self.cut_height, &base.cut_height) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.building, later.building);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.level, later.level);
        merge_slot(&mut self.height, later.height);
        merge_slot(&mut self.cut_height, later.cut_height);
    }
    fn minimal(&self, base: &Storey) -> Self {
        Self { building: changed(&self.building, &base.building), name: changed(&self.name, &base.name), level: changed(&self.level, &base.level), height: changed(&self.height, &base.height), cut_height: changed_assigned(&self.cut_height, &base.cut_height) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.building.is_some() {
            out.push("building".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.level.is_some() {
            out.push("level".to_string());
        }
        if self.height.is_some() {
            out.push("height".to_string());
        }
        if self.cut_height.is_some() {
            out.push("cut_height".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`GridLine`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct GridLinePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub end: Option<Point2>,
}

impl Patch<GridLine> for GridLinePatch {
    fn write(&self, base: &GridLine) -> GridLine {
        GridLine { building: take(&self.building, &base.building), label: take(&self.label, &base.label), start: take(&self.start, &base.start), end: take(&self.end, &base.end) }
    }
    fn restoring(&self, base: &GridLine) -> Self {
        Self { building: restore(&self.building, &base.building), label: restore(&self.label, &base.label), start: restore(&self.start, &base.start), end: restore(&self.end, &base.end) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.building, later.building);
        merge_slot(&mut self.label, later.label);
        merge_slot(&mut self.start, later.start);
        merge_slot(&mut self.end, later.end);
    }
    fn minimal(&self, base: &GridLine) -> Self {
        Self { building: changed(&self.building, &base.building), label: changed(&self.label, &base.label), start: changed(&self.start, &base.start), end: changed(&self.end, &base.end) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.building.is_some() {
            out.push("building".to_string());
        }
        if self.label.is_some() {
            out.push("label".to_string());
        }
        if self.start.is_some() {
            out.push("start".to_string());
        }
        if self.end.is_some() {
            out.push("end".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Space`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SpacePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub boundary: Option<SpaceBoundary>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub zone: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub floor_finish: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub wall_finish: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub ceiling_finish: Option<Assigned<Option<String>>>,
}

impl Patch<Space> for SpacePatch {
    fn write(&self, base: &Space) -> Space {
        Space { storey: take(&self.storey, &base.storey), number: take(&self.number, &base.number), name: take(&self.name, &base.name), boundary: take(&self.boundary, &base.boundary), usage: take(&self.usage, &base.usage), phase: take(&self.phase, &base.phase), zone: take_assigned(&self.zone, &base.zone), floor_finish: take_assigned(&self.floor_finish, &base.floor_finish), wall_finish: take_assigned(&self.wall_finish, &base.wall_finish), ceiling_finish: take_assigned(&self.ceiling_finish, &base.ceiling_finish) }
    }
    fn restoring(&self, base: &Space) -> Self {
        Self { storey: restore(&self.storey, &base.storey), number: restore(&self.number, &base.number), name: restore(&self.name, &base.name), boundary: restore(&self.boundary, &base.boundary), usage: restore(&self.usage, &base.usage), phase: restore(&self.phase, &base.phase), zone: restore_assigned(&self.zone, &base.zone), floor_finish: restore_assigned(&self.floor_finish, &base.floor_finish), wall_finish: restore_assigned(&self.wall_finish, &base.wall_finish), ceiling_finish: restore_assigned(&self.ceiling_finish, &base.ceiling_finish) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.number, later.number);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.boundary, later.boundary);
        merge_slot(&mut self.usage, later.usage);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.zone, later.zone);
        merge_slot(&mut self.floor_finish, later.floor_finish);
        merge_slot(&mut self.wall_finish, later.wall_finish);
        merge_slot(&mut self.ceiling_finish, later.ceiling_finish);
    }
    fn minimal(&self, base: &Space) -> Self {
        Self { storey: changed(&self.storey, &base.storey), number: changed(&self.number, &base.number), name: changed(&self.name, &base.name), boundary: changed(&self.boundary, &base.boundary), usage: changed(&self.usage, &base.usage), phase: changed(&self.phase, &base.phase), zone: changed_assigned(&self.zone, &base.zone), floor_finish: changed_assigned(&self.floor_finish, &base.floor_finish), wall_finish: changed_assigned(&self.wall_finish, &base.wall_finish), ceiling_finish: changed_assigned(&self.ceiling_finish, &base.ceiling_finish) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.number.is_some() {
            out.push("number".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.boundary.is_some() {
            out.push("boundary".to_string());
        }
        if self.usage.is_some() {
            out.push("usage".to_string());
        }
        if self.phase.is_some() {
            out.push("phase".to_string());
        }
        if self.zone.is_some() {
            out.push("zone".to_string());
        }
        if self.floor_finish.is_some() {
            out.push("floor_finish".to_string());
        }
        if self.wall_finish.is_some() {
            out.push("wall_finish".to_string());
        }
        if self.ceiling_finish.is_some() {
            out.push("ceiling_finish".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Zone`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ZonePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub occupancy_density: Option<f64>,
}

impl Patch<Zone> for ZonePatch {
    fn write(&self, base: &Zone) -> Zone {
        Zone { name: take(&self.name, &base.name), category: take(&self.category, &base.category), occupancy_density: take(&self.occupancy_density, &base.occupancy_density) }
    }
    fn restoring(&self, base: &Zone) -> Self {
        Self { name: restore(&self.name, &base.name), category: restore(&self.category, &base.category), occupancy_density: restore(&self.occupancy_density, &base.occupancy_density) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.category, later.category);
        merge_slot(&mut self.occupancy_density, later.occupancy_density);
    }
    fn minimal(&self, base: &Zone) -> Self {
        Self { name: changed(&self.name, &base.name), category: changed(&self.category, &base.category), occupancy_density: changed(&self.occupancy_density, &base.occupancy_density) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.category.is_some() {
            out.push("category".to_string());
        }
        if self.occupancy_density.is_some() {
            out.push("occupancy_density".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`AreaScheme`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct AreaSchemePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub measure: Option<AreaMeasure>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

impl Patch<AreaScheme> for AreaSchemePatch {
    fn write(&self, base: &AreaScheme) -> AreaScheme {
        AreaScheme { name: take(&self.name, &base.name), measure: take(&self.measure, &base.measure), usages: take(&self.usages, &base.usages), zones: take(&self.zones, &base.zones) }
    }
    fn restoring(&self, base: &AreaScheme) -> Self {
        Self { name: restore(&self.name, &base.name), measure: restore(&self.measure, &base.measure), usages: restore(&self.usages, &base.usages), zones: restore(&self.zones, &base.zones) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.measure, later.measure);
        merge_slot(&mut self.usages, later.usages);
        merge_slot(&mut self.zones, later.zones);
    }
    fn minimal(&self, base: &AreaScheme) -> Self {
        Self { name: changed(&self.name, &base.name), measure: changed(&self.measure, &base.measure), usages: changed(&self.usages, &base.usages), zones: changed(&self.zones, &base.zones) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.measure.is_some() {
            out.push("measure".to_string());
        }
        if self.usages.is_some() {
            out.push("usages".to_string());
        }
        if self.zones.is_some() {
            out.push("zones".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

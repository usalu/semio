import { readFileSync, writeFileSync } from "fs";

const path = process.argv[2];
let src = readFileSync(path, "utf8");

// Make KernelCurve Clone
src = src.replace("enum KernelCurve {", "#[derive(Clone)]\nenum KernelCurve {");

// Replace broken StoredEntity round-trip with material-pack encoding (engine key only).
const storedBlockStart = src.indexOf("#[derive(Clone, serde::Serialize, serde::Deserialize)]\nstruct StoredEntry {");
const storedBlockEnd = src.indexOf("fn entity_material(entity: &Entity)", storedBlockStart);
if (storedBlockStart < 0) {
  // already may be after entity_material — find StoredEntry differently
}
const fromEntryImplStart = src.indexOf("impl StoredEntry {");
const intoEntityEndMarker = src.indexOf("// #endregion 🔖️Registry");
// Safer: change register_entity to not use StoredEntry::from_entry

src = src.replace(
  `fn register_entity(&mut self, kind: GeometryKind, entity: Entity) -> Result<GeometryHandle, BrepError> {
        let stored = StoredEntry::from_entry(self.session_id, &Entry { kind, entity: entity.clone() })?;
        let pack = StoredEntry::encode_pack(&stored).map_err(BrepError::InvalidInput)?;
        let kernel_handle = self.cache.derive(BREP_ENTITY_ENGINE_ID, &pack).map_err(map_engine_fault)?;
        let geometry = geometry_handle_from_key(kernel_handle.key);
        self.live.insert(geometry.as_str().to_string());
        self.entity_lut.insert(kernel_handle.key, Entry { kind, entity });
        Ok(geometry)
    }`,
  `fn register_entity(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        let mut pack = self.session_id.to_vec();
        pack.push(0);
        pack.extend_from_slice(format!("{kind:?}").as_bytes());
        pack.push(0);
        pack.extend_from_slice(&Self::entity_material(&entity));
        let kernel_handle = self.cache.derive(BREP_ENTITY_ENGINE_ID, &pack).expect("brep entity engine registered");
        let geometry = geometry_handle_from_key(kernel_handle.key);
        self.live.insert(geometry.as_str().to_string());
        self.entity_lut.insert(kernel_handle.key, Entry { kind, entity });
        geometry
    }`,
);

src = src.replace(
  `fn register_solid(&mut self, solid: SolidId) -> Result<GeometryHandle, BrepError> {
        self.register_entity(GeometryKind::Solid, Entity::Solid(solid))
    }`,
  `fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.register_entity(GeometryKind::Solid, Entity::Solid(solid))
    }`,
);

// Restore Ok(self.register_*) wrappers since register is infallible again
src = src.replace(/([^?])self\.register_(solid|entity)\(/g, (m, pre, name) => {
  // don't double-wrap if already Ok(
  return `${pre}self.register_${name}(`;
});

// Fix collects that were Result-ized
src = src.replace(
  ".map(|s| self.register_solid(s)).collect::<Result<Vec<_>, _>>()?",
  ".map(|s| self.register_solid(s)).collect()",
);

// Fix remaining Ok(self.register_entity( multiline — make Ok(...) around handle again for Result-returning methods
// Lines like: Ok(self.register_entity(...)) when register returns GeometryHandle is correct.
// Fix bare `self.register_entity` that should be Ok(...) inside Result-returning fns — cargo will tell us.

// Fix CurveNurbs into_entity missing weights — add weights field use if still present
src = src.replace(
  "Self::CurveNurbs { degree, knots, poles } => {",
  "Self::CurveNurbs { degree, knots, poles, weights } => {",
);
src = src.replace(
  "let curve = NurbsCurve::from_knots_control_points(degree, knots, points).map_err(|e| BrepError::Operation(e.to_string()))?;",
  "let curve = NurbsCurve::from_knots_control_points(degree, knots, points, weights).map_err(|e| BrepError::Operation(e.to_string()))?;",
);

writeFileSync(path, src);
console.log("simplified register_entity");

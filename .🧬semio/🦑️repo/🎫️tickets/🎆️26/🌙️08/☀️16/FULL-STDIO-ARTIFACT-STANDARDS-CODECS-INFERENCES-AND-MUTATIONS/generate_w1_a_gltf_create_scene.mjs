import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const root = join(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations', 'create-scene');
const id = 's.stdio.gltf.mutation.create-scene.v1';
const json = value => `${JSON.stringify(value, null, 2)}\n`;
const mutationTs = `/** 🦠️ Creates one empty top-level glTF scene at an explicit position. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, insert, position, type GltfMutationRejection, type GltfStructuralResult } from '../../🔒️top-level-collections-private/🟦️component.ts';
export const GltfCreateSceneDescriptor = { id: '${id}', version: 1, touchedPathPattern: 'document/scenes/{position}' } as const;
export interface GltfCreateScenePayload { position: number }
export const validateGltfCreateScene = (payload: GltfCreateScenePayload, base: GltfSnapshot): GltfMutationRejection | undefined => position(payload.position, base.document.scenes.length, 'document/scenes', true);
export const applyGltfCreateScene = (base: GltfSnapshot, payload: GltfCreateScenePayload): GltfStructuralResult => { const rejection = validateGltfCreateScene(payload, base); if (rejection) return { accepted: false, rejection }; const snapshot = clone(base); const scene: GltfScene = { nodes: [] }; insert(snapshot, 'scenes', payload.position, scene); return { accepted: true, snapshot: clone(snapshot) }; };
`;
const diffTs = `/** 🔺️ Exact create-scene insertion delta with forward stale-state protection. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, insert, position, reject, same, type GltfMutationRejection, type GltfStructuralResult } from '../../🔒️top-level-collections-private/🟦️component.ts';
export const GltfCreateSceneDiffDescriptor = { id: '${id}', version: 1, phase: 'diff', touchedPathPatterns: ['document/scenes/{position}', 'document/scene'] } as const;
export interface GltfCreateSceneDiff { readonly id: '${id}'; readonly version: 1; readonly phase: 'diff'; readonly touchedPaths: readonly string[]; readonly position: number; readonly expectedSceneCount: number; readonly expectedDefaultSceneBefore: number | null; readonly expectedNextScene: GltfScene | null; readonly scene: GltfScene; }
const defaultAfter = (defaultScene: number | undefined, position: number): number | undefined => defaultScene === undefined ? undefined : defaultScene >= position ? defaultScene + 1 : defaultScene;
const paths = (base: GltfSnapshot, position: number): readonly string[] => base.document.scene === defaultAfter(base.document.scene, position) ? [\`document/scenes/\${position}\`] : [\`document/scenes/\${position}\`, 'document/scene'];
const expectedScene = (): GltfScene => ({ nodes: [] });
const stale = (path: string, detail: string): GltfMutationRejection => reject('gltf.mutation.stale-diff', path, detail);
export const validateGltfCreateSceneDiff = (diff: GltfCreateSceneDiff, base: GltfSnapshot): GltfMutationRejection | undefined => { if (diff.id !== GltfCreateSceneDiffDescriptor.id || diff.version !== 1 || diff.phase !== 'diff') return reject('gltf.mutation.invalid-diff-envelope', 'diff', 'canonical identity or phase does not match'); const range = position(diff.position, base.document.scenes.length, 'document/scenes', true); if (range) return range; if (diff.expectedSceneCount !== base.document.scenes.length) return stale('diff/expectedSceneCount', 'scene collection no longer matches the planned pre-state'); if (diff.expectedDefaultSceneBefore !== (base.document.scene ?? null)) return stale('document/scene', 'default scene no longer matches the planned pre-state'); if (!same(diff.expectedNextScene, base.document.scenes[diff.position] ?? null)) return stale(\`document/scenes/\${diff.position}\`, 'insertion anchor no longer matches the planned pre-state'); if (!same(diff.touchedPaths, paths(base, diff.position))) return reject('gltf.mutation.invalid-touched-paths', 'diff/touchedPaths', 'paths must name every concrete changed location'); return same(diff.scene, expectedScene()) ? undefined : reject('gltf.mutation.invalid-created-scene', 'diff/scene', 'create-scene may only insert the canonical empty scene'); };
export const applyGltfCreateSceneDiff = (base: GltfSnapshot, diff: GltfCreateSceneDiff): GltfStructuralResult => { const rejection = validateGltfCreateSceneDiff(diff, base); if (rejection) return { accepted: false, rejection }; const snapshot = clone(base); insert(snapshot, 'scenes', diff.position, structuredClone(diff.scene)); return { accepted: true, snapshot: clone(snapshot) }; };
export const encodeGltfCreateSceneDiff = (diff: GltfCreateSceneDiff): string => JSON.stringify(diff);
export const deriveGltfCreateSceneDiff = (base: GltfSnapshot, payload: { position: number }): { accepted: true; diff: GltfCreateSceneDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const range = position(payload.position, base.document.scenes.length, 'document/scenes', true); if (range) return { accepted: false, rejection: range }; const touchedPaths = paths(base, payload.position); return { accepted: true, diff: { id: '${id}', version: 1, phase: 'diff', touchedPaths, position: payload.position, expectedSceneCount: base.document.scenes.length, expectedDefaultSceneBefore: base.document.scene ?? null, expectedNextScene: structuredClone(base.document.scenes[payload.position] ?? null), scene: expectedScene() }, touchedPaths }; };
`;
const inverseTs = `/** ↩️ Exact create-scene removal inverse with complete default-scene restoration. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, position, reject, remove, same, type GltfMutationRejection, type GltfStructuralResult } from '../../🔒️top-level-collections-private/🟦️component.ts';
export const GltfCreateSceneInverseDescriptor = { id: '${id}', version: 1, phase: 'inverse', touchedPathPatterns: ['document/scenes/{position}', 'document/scene'] } as const;
export interface GltfCreateSceneInverse { readonly id: '${id}'; readonly version: 1; readonly phase: 'inverse'; readonly touchedPaths: readonly string[]; readonly position: number; readonly expectedScene: GltfScene; readonly defaultSceneBefore: number | null; readonly expectedDefaultSceneAfter: number | null; }
const defaultAfter = (defaultScene: number | undefined, position: number): number | undefined => defaultScene === undefined ? undefined : defaultScene >= position ? defaultScene + 1 : defaultScene;
const paths = (position: number, before: number | null, after: number | null): readonly string[] => before === after ? [\`document/scenes/\${position}\`] : [\`document/scenes/\${position}\`, 'document/scene'];
const expectedScene = (): GltfScene => ({ nodes: [] });
export const validateGltfCreateSceneInverse = (inverse: GltfCreateSceneInverse, after: GltfSnapshot): GltfMutationRejection | undefined => { if (inverse.id !== GltfCreateSceneInverseDescriptor.id || inverse.version !== 1 || inverse.phase !== 'inverse') return reject('gltf.mutation.invalid-inverse-envelope', 'inverse', 'canonical identity or phase does not match'); const range = position(inverse.position, after.document.scenes.length, 'document/scenes'); if (range) return range; if (!same(inverse.touchedPaths, paths(inverse.position, inverse.defaultSceneBefore, inverse.expectedDefaultSceneAfter))) return reject('gltf.mutation.invalid-touched-paths', 'inverse/touchedPaths', 'paths must name every concrete changed location'); if (!same(inverse.expectedScene, expectedScene())) return reject('gltf.mutation.invalid-created-scene', 'inverse/expectedScene', 'inverse must target the canonical empty scene'); if (inverse.expectedDefaultSceneAfter !== (after.document.scene ?? null)) return reject('gltf.mutation.stale-inverse', 'document/scene', 'default scene does not match the forward-created state'); return same(after.document.scenes[inverse.position], inverse.expectedScene) ? undefined : reject('gltf.mutation.stale-inverse', \`document/scenes/\${inverse.position}\`, 'current scene does not match the forward-created scene'); };
export const applyGltfCreateSceneInverse = (after: GltfSnapshot, inverse: GltfCreateSceneInverse): GltfStructuralResult => { const rejection = validateGltfCreateSceneInverse(inverse, after); if (rejection) return { accepted: false, rejection }; const snapshot = clone(after); remove(snapshot, 'scenes', inverse.position); if (inverse.defaultSceneBefore === null) delete snapshot.document.scene; else snapshot.document.scene = inverse.defaultSceneBefore; return { accepted: true, snapshot: clone(snapshot) }; };
export const encodeGltfCreateSceneInverse = (inverse: GltfCreateSceneInverse): string => JSON.stringify(inverse);
export const deriveGltfCreateSceneInverse = (base: GltfSnapshot, payload: { position: number }): { accepted: true; inverse: GltfCreateSceneInverse; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const range = position(payload.position, base.document.scenes.length, 'document/scenes', true); if (range) return { accepted: false, rejection: range }; const defaultSceneBefore = base.document.scene ?? null; const expectedDefaultSceneAfter = defaultAfter(base.document.scene, payload.position) ?? null; const touchedPaths = paths(payload.position, defaultSceneBefore, expectedDefaultSceneAfter); return { accepted: true, inverse: { id: '${id}', version: 1, phase: 'inverse', touchedPaths, position: payload.position, expectedScene: expectedScene(), defaultSceneBefore, expectedDefaultSceneAfter }, touchedPaths }; };
`;
const mutationRust = `//! 🦠️ Creates one empty top-level glTF scene at an explicit position.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "${id}";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateScenePayload { pub position: usize }
pub fn validate(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.scenes.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection")); } Ok(()) }
pub fn apply(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(payload.position))?; next.document.scenes.insert(payload.position, GltfScene::default()); Ok(next) }
`;
const diffRust = `//! 🔺️ Exact create-scene insertion delta with forward stale-state protection.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "${id}";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub enum GltfCreateSceneDiffPhase { Diff }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateSceneDiff { pub id: String, pub version: u32, pub phase: GltfCreateSceneDiffPhase, pub touched_paths: Vec<String>, pub position: usize, pub expected_scene_count: usize, pub expected_default_scene_before: Option<usize>, pub expected_next_scene: Option<GltfScene>, pub scene: GltfScene }
fn default_after(default_scene: Option<usize>, position: usize) -> Result<Option<usize>, GltfTopLevelMutationRejection> { default_scene.map(|scene| if scene >= position { scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond usize")) } else { Ok(scene) }).transpose() }
fn paths(base: &GltfSnapshot, position: usize) -> Result<Vec<String>, GltfTopLevelMutationRejection> { Ok(if base.document.scene == default_after(base.document.scene, position)? { vec![format!("document/scenes/{}", position)] } else { vec![format!("document/scenes/{}", position), "document/scene".into()] }) }
pub fn validate(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 || diff.phase != GltfCreateSceneDiffPhase::Diff { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "canonical identity or phase does not match")); } if diff.position > base.document.scenes.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection")); } if diff.expected_scene_count != base.document.scenes.len() { return Err(reject("gltf.mutation.stale-diff", "diff/expectedSceneCount", "scene collection no longer matches the planned pre-state")); } if diff.expected_default_scene_before != base.document.scene { return Err(reject("gltf.mutation.stale-diff", "document/scene", "default scene no longer matches the planned pre-state")); } if diff.expected_next_scene != base.document.scenes.get(diff.position).cloned() { return Err(reject("gltf.mutation.stale-diff", format!("document/scenes/{}", diff.position), "insertion anchor no longer matches the planned pre-state")); } if diff.touched_paths != paths(base, diff.position)? { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must name every concrete changed location")); } if diff.scene != GltfScene::default() { return Err(reject("gltf.mutation.invalid-created-scene", "diff/scene", "create-scene may only insert the canonical empty scene")); } Ok(()) }
pub fn apply(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(diff, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(diff.position))?; next.document.scenes.insert(diff.position, diff.scene.clone()); Ok(next) }
pub fn encode(diff: &GltfCreateSceneDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub fn derive(base: &GltfSnapshot, position: usize) -> Result<GltfCreateSceneDiff, GltfTopLevelMutationRejection> { if position > base.document.scenes.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection")); } Ok(GltfCreateSceneDiff { id: ID.into(), version: 1, phase: GltfCreateSceneDiffPhase::Diff, touched_paths: paths(base, position)?, position, expected_scene_count: base.document.scenes.len(), expected_default_scene_before: base.document.scene, expected_next_scene: base.document.scenes.get(position).cloned(), scene: GltfScene::default() }) }
`;
const inverseRust = `//! ↩️ Exact create-scene removal inverse with complete default-scene restoration.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, scenes_op, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "${id}";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub enum GltfCreateSceneInversePhase { Inverse }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateSceneInverse { pub id: String, pub version: u32, pub phase: GltfCreateSceneInversePhase, pub touched_paths: Vec<String>, pub position: usize, pub expected_scene: GltfScene, pub default_scene_before: Option<usize>, pub expected_default_scene_after: Option<usize> }
fn default_after(default_scene: Option<usize>, position: usize) -> Result<Option<usize>, GltfTopLevelMutationRejection> { default_scene.map(|scene| if scene >= position { scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond usize")) } else { Ok(scene) }).transpose() }
fn paths(position: usize, before: Option<usize>, after: Option<usize>) -> Vec<String> { if before == after { vec![format!("document/scenes/{}", position)] } else { vec![format!("document/scenes/{}", position), "document/scene".into()] } }
pub fn validate(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if inverse.id != ID || inverse.version != 1 || inverse.phase != GltfCreateSceneInversePhase::Inverse { return Err(reject("gltf.mutation.invalid-inverse-envelope", "inverse", "canonical identity or phase does not match")); } if inverse.position >= after.document.scenes.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "position must address the created scene")); } if inverse.touched_paths != paths(inverse.position, inverse.default_scene_before, inverse.expected_default_scene_after) { return Err(reject("gltf.mutation.invalid-touched-paths", "inverse/touchedPaths", "paths must name every concrete changed location")); } if inverse.expected_scene != GltfScene::default() { return Err(reject("gltf.mutation.invalid-created-scene", "inverse/expectedScene", "inverse must target the canonical empty scene")); } if inverse.expected_default_scene_after != after.document.scene { return Err(reject("gltf.mutation.stale-inverse", "document/scene", "default scene does not match the forward-created state")); } if after.document.scenes[inverse.position] != inverse.expected_scene { return Err(reject("gltf.mutation.stale-inverse", format!("document/scenes/{}", inverse.position), "current scene does not match the forward-created scene")); } Ok(()) }
pub fn apply(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(inverse, after)?; let mut next = after.clone(); scenes_op(&mut next, GltfTopLevelFamily::Scenes, inverse.position, None, None)?; next.document.scene = inverse.default_scene_before; Ok(next) }
pub fn encode(inverse: &GltfCreateSceneInverse) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(inverse).map_err(|error| reject("gltf.mutation.encode-failed", "inverse", error.to_string())) }
pub fn derive(base: &GltfSnapshot, position: usize) -> Result<GltfCreateSceneInverse, GltfTopLevelMutationRejection> { if position > base.document.scenes.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection")); } let default_scene_before = base.document.scene; let expected_default_scene_after = default_after(default_scene_before, position)?; Ok(GltfCreateSceneInverse { id: ID.into(), version: 1, phase: GltfCreateSceneInversePhase::Inverse, touched_paths: paths(position, default_scene_before, expected_default_scene_after), position, expected_scene: GltfScene::default(), default_scene_before, expected_default_scene_after }) }
`;
const payloadSchema = { type: 'object', additionalProperties: false, properties: { position: { type: 'integer', minimum: 0 } }, required: ['position'] };
const sceneSchema = { type: 'object', additionalProperties: false, properties: { nodes: { type: 'array', maxItems: 0, items: { type: 'integer' } } }, required: ['nodes'] };
const anchorSceneSchema = { type: 'object', additionalProperties: false, properties: { nodes: { type: 'array', items: { type: 'integer', minimum: 0 } }, name: { type: 'string' }, extensions: {}, extras: {} }, required: ['nodes'] };
const nullableIndex = { anyOf: [{ type: 'integer', minimum: 0 }, { type: 'null' }] };
const nullableAnchorScene = { anyOf: [anchorSceneSchema, { type: 'null' }] };
const touchedPath = { anyOf: [{ const: 'document/scene' }, { type: 'string', pattern: '^document/scenes/[0-9]+$' }] };
const envelope = phase => ({ $schema: 'https://json-schema.org/draft/2020-12/schema', $id: id, title: `GltfCreateScene${phase[0].toUpperCase()}${phase.slice(1)}V1`, type: 'object', additionalProperties: false, properties: { id: { const: id }, version: { const: 1 }, phase: { const: phase }, touchedPaths: { type: 'array', minItems: 1, maxItems: 2, items: touchedPath }, position: { type: 'integer', minimum: 0 }, ...(phase === 'diff' ? { expectedSceneCount: { type: 'integer', minimum: 0 }, expectedDefaultSceneBefore: nullableIndex, expectedNextScene: nullableAnchorScene, scene: sceneSchema } : { expectedScene: sceneSchema, defaultSceneBefore: nullableIndex, expectedDefaultSceneAfter: nullableIndex }) }, required: ['id','version','phase','touchedPaths','position', ...(phase === 'diff' ? ['expectedSceneCount','expectedDefaultSceneBefore','expectedNextScene','scene'] : ['expectedScene','defaultSceneBefore','expectedDefaultSceneAfter'])], 'x-semio': { id, version: 1, phase, touchedPathPatterns: ['document/scenes/{position}', 'document/scene'] } });
const graphql = phase => phase === 'mutation' ? `input GltfCreateScenePayloadV1 { position: Int! }\n` : phase === 'diff' ? `scalar GltfJson\nenum GltfCreateSceneDiffPhaseV1 { DIFF }\ntype GltfCreateSceneValueV1 { nodes: [Int!]! }\ntype GltfCreateSceneAnchorV1 { nodes: [Int!]!, name: String, extensions: GltfJson, extras: GltfJson }\ntype GltfCreateSceneDiffV1 { id: ID!, version: Int!, phase: GltfCreateSceneDiffPhaseV1!, touchedPaths: [String!]!, position: Int!, expectedSceneCount: Int!, expectedDefaultSceneBefore: Int, expectedNextScene: GltfCreateSceneAnchorV1, scene: GltfCreateSceneValueV1! }\n` : `enum GltfCreateSceneInversePhaseV1 { INVERSE }\ntype GltfCreateSceneValueV1 { nodes: [Int!]! }\ntype GltfCreateSceneInverseV1 { id: ID!, version: Int!, phase: GltfCreateSceneInversePhaseV1!, touchedPaths: [String!]!, position: Int!, expectedScene: GltfCreateSceneValueV1!, defaultSceneBefore: Int, expectedDefaultSceneAfter: Int }\n`;
const proto = phase => phase === 'mutation' ? `syntax = "proto3";\npackage stdio.gltf.mutation;\nmessage GltfCreateScenePayloadV1 { uint32 position = 1; }\n` : phase === 'diff' ? `syntax = "proto3";\npackage stdio.gltf.mutation;\nenum GltfCreateSceneDiffPhaseV1 { GLTF_CREATE_SCENE_DIFF_PHASE_V1_UNSPECIFIED = 0; GLTF_CREATE_SCENE_DIFF_PHASE_V1_DIFF = 1; }\nmessage GltfCreateSceneJsonArrayV1 { repeated GltfCreateSceneJsonV1 values = 1; }\nmessage GltfCreateSceneJsonObjectFieldV1 { string key = 1; GltfCreateSceneJsonV1 value = 2; }\nmessage GltfCreateSceneJsonObjectV1 { repeated GltfCreateSceneJsonObjectFieldV1 fields = 1; }\nmessage GltfCreateSceneJsonV1 { oneof value { bool null_value = 1; bool boolean_value = 2; double number_value = 3; string string_value = 4; GltfCreateSceneJsonArrayV1 array_value = 5; GltfCreateSceneJsonObjectV1 object_value = 6; } }\nmessage GltfCreateSceneValueV1 { repeated uint32 nodes = 1; }\nmessage GltfCreateSceneAnchorV1 { repeated uint32 nodes = 1; optional string name = 2; GltfCreateSceneJsonV1 extensions = 3; GltfCreateSceneJsonV1 extras = 4; }\nmessage GltfCreateSceneDiffV1 { string id = 1; uint32 version = 2; GltfCreateSceneDiffPhaseV1 phase = 3; repeated string touched_paths = 4; uint32 position = 5; GltfCreateSceneValueV1 scene = 6; uint32 expected_scene_count = 7; optional uint32 expected_default_scene_before = 8; GltfCreateSceneAnchorV1 expected_next_scene = 9; }\n` : `syntax = "proto3";\npackage stdio.gltf.mutation;\nenum GltfCreateSceneInversePhaseV1 { GLTF_CREATE_SCENE_INVERSE_PHASE_V1_UNSPECIFIED = 0; GLTF_CREATE_SCENE_INVERSE_PHASE_V1_INVERSE = 1; }\nmessage GltfCreateSceneValueV1 { repeated uint32 nodes = 1; }\nmessage GltfCreateSceneInverseV1 { string id = 1; uint32 version = 2; GltfCreateSceneInversePhaseV1 phase = 3; repeated string touched_paths = 4; uint32 position = 5; GltfCreateSceneValueV1 expected_scene = 6; optional uint32 default_scene_before = 7; optional uint32 expected_default_scene_after = 8; }\n`;
const contract = { id, vectors: [{ name: 'insertsAtZeroAndRemapsDefaultScene', base: { scene: 0, scenes: [{ nodes: [] }] }, payload: { position: 0 }, after: { scene: 1, scenes: [{ nodes: [] }, { nodes: [] }] }, undo: { scene: 0, scenes: [{ nodes: [] }] }, diff: { id, version: 1, phase: 'diff', touchedPaths: ['document/scenes/0', 'document/scene'], position: 0, expectedSceneCount: 1, expectedDefaultSceneBefore: 0, expectedNextScene: { nodes: [] }, scene: { nodes: [] } }, inverse: { id, version: 1, phase: 'inverse', touchedPaths: ['document/scenes/0', 'document/scene'], position: 0, expectedScene: { nodes: [] }, defaultSceneBefore: 0, expectedDefaultSceneAfter: 1 }, rejections: [{ name: 'outOfRangePosition', payload: { position: 2 }, code: 'gltf.mutation.insert-out-of-range' }, { name: 'staleDiffReplay', code: 'gltf.mutation.stale-diff' }, { name: 'staleDefaultScene', code: 'gltf.mutation.stale-diff' }, { name: 'staleInsertionAnchor', scene: { nodes: [0] }, code: 'gltf.mutation.stale-diff' }, { name: 'staleInverse', scene: { nodes: [0] }, code: 'gltf.mutation.stale-inverse' }] }], laws: ['acceptance','typed-rejection','concrete-touched-path','diff-apply','stale-diff-rejection','stale-default-scene-rejection','stale-insertion-anchor-rejection','inverse-restores-base','stale-inverse-rejection','json-serialization','canonical-id-across-phases'] };
const contractTs = `/** 🧪️ Executes create-scene laws from the canonical JSON vector. */
import assert from 'node:assert/strict';
import contractJson from './🔣️component.json' with { type: 'json' };
import { applyGltfCreateScene } from '../🦠️mutation/🟦️component.ts';
import { applyGltfCreateSceneDiff, deriveGltfCreateSceneDiff, encodeGltfCreateSceneDiff } from '../🔺️diff/🟦️component.ts';
import { applyGltfCreateSceneInverse, deriveGltfCreateSceneInverse, encodeGltfCreateSceneInverse } from '../↩️inverse/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
export const gltfCreateSceneContract = contractJson;
export const runGltfCreateSceneContract = (): void => {
  const vector = gltfCreateSceneContract.vectors[0]!;
  const base: GltfSnapshot = { schema: 'gltf/2.0', sourceForm: 'json', buffers: [], document: { asset: { version: '2.0' }, scene: vector.base.scene, scenes: vector.base.scenes, nodes: [], meshes: [], accessors: [], bufferViews: [], buffers: [], materials: [], textures: [], images: [], samplers: [], skins: [], animations: [], cameras: [], extensionsUsed: [], extensionsRequired: [] } };
  const applied = applyGltfCreateScene(base, vector.payload);
  assert.equal(applied.accepted, true, 'accepts the shared vector');
  assert.deepEqual({ scene: applied.snapshot.document.scene, scenes: applied.snapshot.document.scenes }, vector.after, 'mutation produces vector after-state');
  const outOfRange = applyGltfCreateScene(base, vector.rejections[0]!.payload!);
  assert.equal(outOfRange.accepted, false, 'rejects range vector');
  assert.equal(outOfRange.rejection.code, vector.rejections[0]!.code, 'range rejection code is stable');
  const forward = deriveGltfCreateSceneDiff(base, vector.payload);
  assert.equal(forward.accepted, true, 'derives vector diff');
  assert.deepEqual(forward.diff, vector.diff, 'diff equals canonical vector');
  assert.deepEqual(forward.touchedPaths, vector.diff.touchedPaths, 'diff path is concrete');
  const replay = applyGltfCreateSceneDiff(base, forward.diff);
  assert.equal(replay.accepted, true, 'applies vector diff');
  assert.deepEqual(replay.snapshot, applied.snapshot, 'diff application equals mutation');
  const staleForward = applyGltfCreateSceneDiff(applied.snapshot, forward.diff);
  assert.equal(staleForward.accepted, false, 'rejects replay against its post-state');
  assert.equal(staleForward.rejection.code, vector.rejections[1]!.code, 'stale diff rejection code is stable');
  const staleDefault = applyGltfCreateSceneDiff({ ...base, document: { ...base.document, scene: undefined } }, forward.diff);
  assert.equal(staleDefault.accepted, false, 'rejects a changed default-scene precondition');
  assert.equal(staleDefault.rejection.code, vector.rejections[2]!.code, 'default-scene stale rejection code is stable');
  const staleAnchor = applyGltfCreateSceneDiff({ ...base, document: { ...base.document, scenes: [vector.rejections[3]!.scene!] } }, forward.diff);
  assert.equal(staleAnchor.accepted, false, 'rejects a changed insertion anchor');
  assert.equal(staleAnchor.rejection.code, vector.rejections[3]!.code, 'anchor stale rejection code is stable');
  assert.deepEqual(JSON.parse(encodeGltfCreateSceneDiff(forward.diff)), vector.diff, 'diff serialization is stable');
  const undo = deriveGltfCreateSceneInverse(base, vector.payload);
  assert.equal(undo.accepted, true, 'derives vector inverse');
  assert.deepEqual(undo.inverse, vector.inverse, 'inverse equals canonical vector');
  const restored = applyGltfCreateSceneInverse(applied.snapshot, undo.inverse);
  assert.equal(restored.accepted, true, 'applies vector inverse');
  assert.deepEqual({ scene: restored.snapshot.document.scene, scenes: restored.snapshot.document.scenes }, vector.undo, 'inverse produces vector undo-state');
  assert.deepEqual(JSON.parse(encodeGltfCreateSceneInverse(undo.inverse)), vector.inverse, 'inverse serialization is stable');
  const stale = applyGltfCreateSceneInverse({ ...applied.snapshot, document: { ...applied.snapshot.document, scenes: [vector.rejections[4]!.scene!, ...applied.snapshot.document.scenes.slice(1)] } }, undo.inverse);
  assert.equal(stale.accepted, false, 'rejects stale inverse vector');
  assert.equal(stale.rejection.code, vector.rejections[4]!.code, 'stale rejection code is stable');
};
`;
const contractRust = `//! 🧪️ Executes create-scene laws from the canonical JSON vector.
#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Value;
    use crate::artifacts::gltf::schema::mutations::create_scene::{diff, inverse, mutation};
    use crate::artifacts::gltf::schema::snapshot::GltfScene;
    use crate::artifacts::gltf::GltfSnapshot;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Contract { vectors: Vec<Vector> }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Vector { base: SceneState, payload: mutation::GltfCreateScenePayload, after: SceneState, undo: SceneState, diff: Value, inverse: Value, rejections: Vec<Rejection> }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SceneState { scene: usize, scenes: Vec<GltfScene> }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Rejection { payload: Option<mutation::GltfCreateScenePayload>, scene: Option<GltfScene>, code: String }
    fn snapshot(state: &SceneState) -> GltfSnapshot { let mut snapshot = GltfSnapshot::default(); snapshot.schema = "gltf/2.0".into(); snapshot.document.scene = Some(state.scene); snapshot.document.scenes = state.scenes.clone(); snapshot }

    #[test]
    fn create_scene_shared_vector_executes_all_laws() {
        let contract: Contract = serde_json::from_str(include_str!("🔣️component.json")).expect("canonical vector decodes");
        let vector = &contract.vectors[0];
        let base = snapshot(&vector.base);
        let after = mutation::apply(&vector.payload, &base).expect("mutation accepts vector");
        assert_eq!(after.document.scene, Some(vector.after.scene));
        assert_eq!(after.document.scenes, vector.after.scenes);
        let range = mutation::apply(vector.rejections[0].payload.as_ref().expect("range payload"), &base).expect_err("range payload rejects");
        assert_eq!(range.code, vector.rejections[0].code);
        let forward: diff::GltfCreateSceneDiff = serde_json::from_value(vector.diff.clone()).expect("diff decodes");
        let planned = diff::derive(&base, vector.payload.position).expect("diff derives");
        assert_eq!(planned, forward);
        assert_eq!(planned.touched_paths, vec!["document/scenes/0", "document/scene"]);
        let replay = diff::apply(&planned, &base).expect("diff applies");
        assert_eq!(replay, after);
        let stale_forward = diff::apply(&planned, &after).expect_err("post-state replay rejects");
        assert_eq!(stale_forward.code, vector.rejections[1].code);
        let mut stale_default = base.clone();
        stale_default.document.scene = None;
        let stale_default = diff::apply(&planned, &stale_default).expect_err("default-scene precondition rejects");
        assert_eq!(stale_default.code, vector.rejections[2].code);
        let mut stale_anchor = base.clone();
        stale_anchor.document.scenes[0] = vector.rejections[3].scene.clone().expect("anchor scene");
        let stale_anchor = diff::apply(&planned, &stale_anchor).expect_err("insertion anchor rejects");
        assert_eq!(stale_anchor.code, vector.rejections[3].code);
        assert_eq!(serde_json::from_slice::<Value>(&diff::encode(&planned).expect("diff encodes")).expect("encoded diff decodes"), vector.diff);
        let undo: inverse::GltfCreateSceneInverse = serde_json::from_value(vector.inverse.clone()).expect("inverse decodes");
        let inverted = inverse::derive(&base, vector.payload.position).expect("inverse derives");
        assert_eq!(inverted, undo);
        let restored = inverse::apply(&inverted, &after).expect("inverse applies");
        assert_eq!(restored.document.scene, Some(vector.undo.scene));
        assert_eq!(restored.document.scenes, vector.undo.scenes);
        assert_eq!(serde_json::from_slice::<Value>(&inverse::encode(&inverted).expect("inverse encodes")).expect("encoded inverse decodes"), vector.inverse);
        let mut stale_after = after.clone();
        stale_after.document.scenes[0] = vector.rejections[4].scene.clone().expect("stale scene");
        let stale = inverse::apply(&inverted, &stale_after).expect_err("stale inverse rejects");
        assert_eq!(stale.code, vector.rejections[4].code);
    }
}
`;
for (const [phase, ts, rust] of [['🦠️mutation', mutationTs, mutationRust], ['🔺️diff', diffTs, diffRust], ['↩️inverse', inverseTs, inverseRust]]) { const logical = phase === '🦠️mutation' ? 'mutation' : phase === '🔺️diff' ? 'diff' : 'inverse'; await mkdir(join(root, phase), { recursive: true }); await writeFile(join(root, phase, '🟦️component.ts'), ts); await writeFile(join(root, phase, '🦀️component.rs'), rust); await writeFile(join(root, phase, '🔣️component.json'), json(logical === 'mutation' ? { $schema: 'https://json-schema.org/draft/2020-12/schema', $id: id, title: 'GltfCreateScenePayloadV1', ...payloadSchema, 'x-semio': { id, version: 1, phase: 'mutation', touchedPathPattern: 'document/scenes/{position}' } } : envelope(logical))); await writeFile(join(root, phase, '🔗️component.graphql'), graphql(logical)); await writeFile(join(root, phase, '🛰️component.proto'), proto(logical)); }
await mkdir(join(root, '🧪️contract'), { recursive: true });
await writeFile(join(root, '🧪️contract', '🔣️component.json'), json(contract));
await writeFile(join(root, '🧪️contract', '🟦️component.ts'), contractTs);
await writeFile(join(root, '🧪️contract', '🦀️component.rs'), contractRust);
console.log(JSON.stringify({ leaf: 'create-scene', physicalFolders: 3, typedFacets: 15, vectors: contract.vectors.length }, null, 2));

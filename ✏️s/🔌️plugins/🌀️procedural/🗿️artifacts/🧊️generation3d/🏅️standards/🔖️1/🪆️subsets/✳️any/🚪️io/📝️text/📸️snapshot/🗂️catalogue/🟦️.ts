import brepPrimitive from "../../../../🧬️schema/🗂️catalogue/🔣️brep-primitive.json";
import brepCurve from "../../../../🧬️schema/🗂️catalogue/🔣️brep-curve.json";
import brepSurface from "../../../../🧬️schema/🗂️catalogue/🔣️brep-surface.json";
import brepSolid from "../../../../🧬️schema/🗂️catalogue/🔣️brep-solid.json";
import brepBoolean from "../../../../🧬️schema/🗂️catalogue/🔣️brep-boolean.json";
import brepFeature from "../../../../🧬️schema/🗂️catalogue/🔣️brep-feature.json";
import brepTransform from "../../../../🧬️schema/🗂️catalogue/🔣️brep-transform.json";
import brepIntersect from "../../../../🧬️schema/🗂️catalogue/🔣️brep-intersect.json";
import brepEvaluate from "../../../../🧬️schema/🗂️catalogue/🔣️brep-evaluate.json";
import brepTopology from "../../../../🧬️schema/🗂️catalogue/🔣️brep-topology.json";
import brepInterchange from "../../../../🧬️schema/🗂️catalogue/🔣️brep-interchange.json";
import meshPrimitive from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-primitive.json";
import meshConvert from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-convert.json";
import meshTransform from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-transform.json";
import meshComponent from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-component.json";
import meshEdit from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-edit.json";
import meshRepair from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-repair.json";
import meshInspect from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-inspect.json";
import meshInterchange from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-interchange.json";
import meshShading from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-shading.json";
import meshUv from "../../../../🧬️schema/🗂️catalogue/🔣️mesh-uv.json";
import analysisMeasure from "../../../../🧬️schema/🗂️catalogue/🔣️analysis-measure.json";
import analysisCheck from "../../../../🧬️schema/🗂️catalogue/🔣️analysis-check.json";
import mathValues from "../../../../🧬️schema/🗂️catalogue/🔣️math-values.json";
import mathArithmetic from "../../../../🧬️schema/🗂️catalogue/🔣️math-arithmetic.json";
import mathVector from "../../../../🧬️schema/🗂️catalogue/🔣️math-vector.json";
import mathList from "../../../../🧬️schema/🗂️catalogue/🔣️math-list.json";

import { buildCatalogue, type Catalogue, type CatalogueCategoryFile } from "../../../../🧬️schema/🗂️catalogue/🟦️.ts";

/** 📚️ Every bundled category file, ordered by palette position. */
export const CATALOGUE_CATEGORY_FILES: readonly CatalogueCategoryFile[] = ([
  brepPrimitive, brepCurve, brepSurface, brepSolid, brepBoolean, brepFeature, brepTransform, brepIntersect, brepEvaluate, brepTopology, brepInterchange,
  meshPrimitive, meshConvert, meshTransform, meshComponent, meshEdit, meshRepair, meshInspect, meshInterchange, meshShading, meshUv,
  analysisMeasure, analysisCheck, mathValues, mathArithmetic, mathVector, mathList,
] as unknown as CatalogueCategoryFile[]).sort((left, right) => left.category.order - right.category.order || (left.category.id < right.category.id ? -1 : 1));

let bundled: Catalogue | undefined;

/** 📦️ The process-wide bundled catalogue, built once. */
export function catalogue(): Catalogue {
  return bundled ??= buildCatalogue(CATALOGUE_CATEGORY_FILES);
}


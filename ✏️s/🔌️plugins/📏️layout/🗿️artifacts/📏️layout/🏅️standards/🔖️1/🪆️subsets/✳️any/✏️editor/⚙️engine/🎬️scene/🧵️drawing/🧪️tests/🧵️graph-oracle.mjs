/** 🔮️ External binary64 matrix multiplication and exact structural equality remain behind a test-only interface. */
import {vectorSourceOracle} from '../../../../../../../../../../../../🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧵️vector/🔮️oracle.mjs';
import {mat2d,glMatrix} from 'gl-matrix';
glMatrix.setMatrixArrayType(Array);
export const drawingGraphOracle={...vectorSourceOracle,compose:(outer,inner)=>Array.from(mat2d.multiply(mat2d.create(),outer,inner))};

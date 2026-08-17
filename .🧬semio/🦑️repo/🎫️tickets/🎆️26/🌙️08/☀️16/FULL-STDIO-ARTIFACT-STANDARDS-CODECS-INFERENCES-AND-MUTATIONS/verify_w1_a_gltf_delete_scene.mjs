import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const root = resolve(process.cwd(), '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations', 'delete-scene');
const contract = await import(pathToFileURL(join(root, '🧪️contract', '🟦️component.ts')).href);
contract.runGltfDeleteSceneContract();
console.log('[DEBUG] w1-a glTF delete-scene: canonical JSON vector executed through TS deletion/reference/diff/inverse/rejection/serialization laws');

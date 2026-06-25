import { readFileSync } from 'fs';
const d = JSON.parse(readFileSync('/workspaces/semio/assets/compose/kit_metabolism.json', 'utf8'));
const nct = d.designs.find(x => x.name === 'Nakagin Capsule Tower');
const connectingGuids = new Set(nct.connections.map(c => c.connecting.piece.guid));
const rootPieces = nct.pieces.filter(p => connectingGuids.has(p.guid) === false);
const childPieces = nct.pieces.filter(p => connectingGuids.has(p.guid));
console.log('Root:', rootPieces.length, rootPieces[0]?.name);
console.log('Child:', childPieces.length);
const ex = childPieces[0];
console.log('Example child piece:', JSON.stringify(ex));
const cn = nct.connections.find(c => c.connecting.piece.guid === ex.guid);
console.log('Parent connection:', JSON.stringify(cn));
const tp = d.types.find(t => t.guid === ex.type.guid);
console.log('Type:', tp ? tp.name + (tp.variant ? ':' + tp.variant : '') : 'unknown');
// Check if root piece is the first in array
console.log('pieces[0] guid:', nct.pieces[0].guid, 'is root:', connectingGuids.has(nct.pieces[0].guid) === false);
console.log('pieces[1] guid:', nct.pieces[1].guid, 'is child:', connectingGuids.has(nct.pieces[1].guid));

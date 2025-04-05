import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';
import { atom, createStore } from 'jotai';
import { Kit, Design, Type } from '@semio/js';
import { get } from 'http';

console.log('Initializing Yjs and Jotai store...');

const yDoc = new Y.Doc();
const indexeddbPersistence = new IndexeddbPersistence('semio-studio', yDoc);
const yKit = yDoc.getMap<Kit>('kit');

export const kitAtom = atom<Kit | null>(
    (get) => yKit.get('kit') ?? null,
);

export const designsAtom = atom((get) => {
    const kit = get(kitAtom);
    if (!kit) {
        return null;
    }
    const designs = new Map<String, Map<String, Design>>();
    kit.designs?.forEach((design) => {
        if (!designs.has(design.name)) {
            designs.set(design.name, new Map<String, Design>());
        }
        designs.get(design.name)?.set(design.variant ?? '', design);
    });
    return designs;
});

export const typesAtom = atom((get) => {
    const kit = get(kitAtom);
    if (!kit) {
        return null;
    }
    const types = new Map<String, Map<String, Type>>();
    kit.types?.forEach((type) => {
        if (!types.has(type.name)) {
            types.set(type.name, new Map<String, Type>());
        }
        types.get(type.name)?.set(type.variant ?? '', type);
    });
    return types;
});

// export const getTypes = (): Map<string, Map<string, Type>> | null => {
//     const kit = yKit.get('kit');
//     if (!kit) {
//         return null;
//     }
//     const types = new Map<string, Map<string, Type>>();
//     kit.types?.forEach((type) => {
//         if (!types.has(type.name)) {
//             types.set(type.name, new Map<string, Type>());
//         }
//         types.get(type.name)?.set(type.variant ?? '', type);
//     });
//     return types;
// };


export async function fetchKit(url: string): Promise<void> {
    console.log('Fetching Kit from URL:', url);
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to fetch: ${response.statusText}`);
        }
        const kit = await response.json() as Kit;

        yDoc.transact(() => {
            yKit.clear();
            yKit.set('kit', kit);
        });
        console.log('Kit successfully fetched and updated from URL:', url);
    } catch (error) {
        console.error('Failed to fetch Kit from URL:', url, error);
        throw error;
    }
}

export const cleanKit = () => {
    console.log('Cleaning Kit...');
    yDoc.transact(() => {
        yKit.clear();
    });
    console.log('Kit cleaned.');
};
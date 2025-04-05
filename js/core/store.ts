import { atom, createStore } from 'jotai'

import { Kit, Design, Type } from '@semio/js'
import { default as metabolism } from '../../assets/semio/kit_metabolism.json'

export const studioStore = createStore()

export const kitAtom = atom<Kit | null>(null)

export const designsAtom = atom((get) => {
    const kit = get(kitAtom);
    if (!kit) {
        return null;
    }
    const designs = new Map<String, Map<String, Design>>();
    kit!.designs?.forEach((design) => {
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
    kit!.types?.forEach((type) => {
        if (!types.has(type.name)) {
            types.set(type.name, new Map<String, Type>());
        }
        types.get(type.name)?.set(type.variant ?? '', type);
    });
    return types;
});
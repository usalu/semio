import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';
import { Kit, Design, Type } from '@semio/js';
import { create } from 'zustand';

console.log('Initializing Yjs and Jotai store...');

const yDoc = new Y.Doc();
const indexeddbPersistence = new IndexeddbPersistence('semio-studio', yDoc);
const yKit = yDoc.getMap<Kit>('kit');

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


export interface KitStore {
    kit: Kit | null;
    setKit: (kit: Kit) => void;
    clearKit: () => void;
}

export const useKit = create<KitStore>((set) => ({
    kit: null,
    setKit: (kit: Kit) => {
        set({ kit });
        yDoc.transact(() => {
            yKit.clear();
            yKit.set('kit', kit);
        });
    },
    clearKit: () => {
        set({ kit: null });
        yDoc.transact(() => {
            yKit.clear();
        });
    },
}));

yKit.observe(() => {
    console.log('Kit updated in Yjs:', yKit.toJSON());
    const kit = yKit.get('kit') || null;
    useKit.setState({ kit });
});

export const useTypes = () => {
    const kit = useKit((state) => state.kit);
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
}

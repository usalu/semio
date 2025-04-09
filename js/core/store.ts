import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';
import { WebrtcProvider } from 'y-webrtc';
import { atom, useAtom } from 'jotai';
import { Kit } from '@semio/js';

console.log('Initializing Yjs and Jotai store...');

// Store class to manage a single room's state
class KitStore {
    private yDoc: Y.Doc;
    private yKit: Y.Map<Kit>;
    private provider: WebrtcProvider;
    public atom: ReturnType<typeof atom<Kit | null>>;

    constructor(roomId: string) {
        this.yDoc = new Y.Doc();
        this.yKit = this.yDoc.getMap<Kit>('kit');
        this.atom = atom<Kit | null>(null);

        // Set up persistence
        new IndexeddbPersistence(`semio-studio-${roomId}`, this.yDoc);

        // Set up WebRTC provider
        this.provider = new WebrtcProvider(roomId, this.yDoc, {
            signaling: ['wss://signaling.yjs.dev'],
            connect: true,
        });

        // Subscribe to changes
        this.yKit.observe(() => {
            const kit = this.yKit.get('kit');
            if (kit) {
                this.atom.onMount = (setAtom) => {
                    setAtom(kit);
                };
            }
        });
    }

    updateKit(kit: Kit) {
        this.yDoc.transact(() => {
            this.yKit.set('kit', kit);
        });
    }

    clearKit() {
        this.yDoc.transact(() => {
            this.yKit.clear();
        });
    }

    destroy() {
        this.provider.destroy();
        this.yDoc.destroy();
    }
}

// Store factory to manage multiple stores
class StoreFactory {
    private stores: Map<string, KitStore> = new Map();

    getStore(roomId: string): KitStore {
        if (!this.stores.has(roomId)) {
            this.stores.set(roomId, new KitStore(roomId));
        }
        return this.stores.get(roomId)!;
    }

    removeStore(roomId: string) {
        const store = this.stores.get(roomId);
        if (store) {
            store.destroy();
            this.stores.delete(roomId);
        }
    }
}

// Create a singleton instance of the store factory
const storeFactory = new StoreFactory();

// Hook to use a specific room's kit
export function useKit(roomId: string) {
    const store = storeFactory.getStore(roomId);
    return useAtom(store.atom);
}

// Function to update a specific room's kit
export function updateKit(roomId: string, kit: Kit) {
    const store = storeFactory.getStore(roomId);
    store.updateKit(kit);
}

// Function to clear a specific room's kit
export function clearKit(roomId: string) {
    const store = storeFactory.getStore(roomId);
    store.clearKit();
}

// Function to remove a store when it's no longer needed
export function removeStore(roomId: string) {
    storeFactory.removeStore(roomId);
}

// Function to fetch and update a specific room's kit
export async function fetchKit(roomId: string, url: string): Promise<void> {
    console.log('Fetching Kit from URL:', url);
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to fetch: ${response.statusText}`);
        }
        const kit = await response.json() as Kit;

        const store = storeFactory.getStore(roomId);
        store.updateKit(kit);
        console.log('Kit successfully fetched and updated from URL:', url);
    } catch (error) {
        console.error('Failed to fetch Kit from URL:', url, error);
        throw error;
    }
}


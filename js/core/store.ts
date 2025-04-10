import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';
import { atom, useAtom } from 'jotai';
import { Kit } from '@semio/js';
import React from 'react';

export const useTypes = () => []

// console.log('Initializing Yjs and Jotai store...');
// const ydoc = new Y.Doc();
// const indexeddbProvider = new IndexeddbPersistence('semio', ydoc);
// const kitsMap = ydoc.getMap('kits');
// const kitsAtom = atom<Map<string, Kit>>(new Map());
// export const useKits = () => {
//     const [kits, setKits] = useAtom(kitsAtom);
//     React.useEffect(() => {
//         const observer = () => {
//             const updatedKits = new Map();
//             kitsMap.forEach((value, key) => {
//                 updatedKits.set(key, value as Kit);
//             });
//             setKits(updatedKits);
//         };
//         kitsMap.observe(observer);
//         observer();
//         return () => {
//             kitsMap.unobserve(observer);
//         };
//     }, [setKits]);

//     const addKit = React.useCallback((id: string, kit: Kit) => {
//         kitsMap.set(id, kit);
//     }, []);

//     const removeKit = React.useCallback((id: string) => {
//         kitsMap.delete(id);
//     }, []);

//     const updateKit = React.useCallback((id: string, kit: Kit) => {
//         kitsMap.set(id, kit);
//     }, []);

//     return {
//         kits,
//         addKit,
//         removeKit,
//         updateKit
//     };
// };
// export const getYDoc = () => ydoc;
// export const getIndexedDBProvider = () => indexeddbProvider;


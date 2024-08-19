import { enableMapSet } from 'immer'
enableMapSet()

import { PayloadAction, configureStore, createAsyncThunk, createSlice } from '@reduxjs/toolkit'
import { Design, DesignInput, Kit, Port, Type, TypeInput } from './semio'

export const loadLocalKit = createAsyncThunk('loadLocalKit', async (directory: string) => {
    if (!directory) {
        return
    }
    // TODO: Inject the ipcRenderer into the store to allow the store to work outside of electron
    const response = await window.electron.ipcRenderer.invoke('load-local-kit', directory)
    return {
        directory,
        kit: response.loadLocalKit.kit as Kit
    }
})

export const kitsSlice = createSlice({
    name: 'kits',
    initialState: {
        kits: new Map<string, Kit>()
    },
    reducers: {
        loadKit: {
            reducer: (state, action) => {
                const { directory, reload } = action.payload
                if (!directory) {
                    return
                }
                const kit = state.kits.get(directory)
                if (kit && !reload) {
                    return
                }
                state.kits.set(directory, kit)
            },
            prepare: (directory, reload = false) => {
                return { payload: { directory, reload } }
            }
        }
    },
    extraReducers: (builder) => {
        builder.addCase(loadLocalKit.fulfilled, (state, action) => {
            if (action.payload.error) {
                console.error(action.payload.error)
                return
            }
            state.kits.set(action.payload.directory, action.payload.kit)
        })
    }
})

export const { loadKit } = kitsSlice.actions

export const selectKits = (state: { kits: { kits: Map<string, Kit> } }): Map<string, Kit> =>
    state.kits.kits

export const selectKit = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string
): Kit | undefined => {
    const kits = state.kits.kits
    return kits.get(directory)
}

export const selectTypes = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string
): Map<string, Map<string, Type>> => {
    // typeName -> typeVariant -> Type
    const kit = selectKit(state, directory)
    if (kit && kit.types) {
        const types = new Map<string, Map<string, Type>>()
        kit.types.forEach((type) => {
            if (!types.has(type.name)) {
                types.set(type.name, new Map<string, Type>())
            }
            types.get(type.name)?.set(type.variant, type)
        })
        return types
    }
    return new Map<string, Map<string, Type>>()
}

export const selectType = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string,
    name: string,
    variant: string
): Type | undefined => {
    const kit = selectKit(state, directory)
    if (kit && kit.types) {
        return kit.types.find((type) => type.name === name && type.variant === variant)
    }
    return undefined
}

export const selectPorts = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string
): Map<string, Map<string, Map<string, Port>>> => {
    // typeName -> typeVariant -> portId -> Port
    const kit = selectKit(state, directory)
    if (kit && kit.types) {
        const ports = new Map<string, Map<string, Map<string, Port>>>()
        kit.types.forEach((type) => {
            if (!ports.has(type.name)) {
                ports.set(type.name, new Map<string, Map<string, Port>>())
            }
            if (!ports.get(type.name)?.has(type.variant)) {
                ports.get(type.name)?.set(type.variant, new Map<string, Port>())
            }
            type.ports.forEach((port) => {
                ports.get(type.name)?.get(type.variant)?.set(port.id, port)
            })
        })
        return ports
    }
    return new Map<string, Map<string, Map<string, Port>>>()
}

export const selectDesigns = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string
): Map<string, Map<string, Design>> => {
    const kit = selectKit(state, directory)
    if (kit && kit.designs) {
        const designs = new Map<string, Map<string, Design>>()
        kit.designs.forEach((design) => {
            if (!designs.has(design.name)) {
                designs.set(design.name, new Map<string, Design>())
            }
            designs.get(design.name)?.set(design.variant, design)
        })
        return designs
    }
    return new Map<string, Map<string, Design>>()
}

export const selectDesign = (
    state: { kits: { kits: Map<string, Kit> } },
    directory: string,
    name: string,
    variant: string
): Design | undefined => {
    const kit = selectKit(state, directory)
    if (kit && kit.designs) {
        return kit.designs.find(
            (design) => design.name === name && design.variant === variant
        )
    }
    return undefined
}

export enum ViewKind {
    Type,
    Design
}

export interface IArtifactView {
    kind: ViewKind
    id: string
    readonly name: string
    readonly description: string
    readonly icon: string
    kitDirectory: string
    toObject(): object
}

export class TypeView implements IArtifactView {
    kind: ViewKind = ViewKind.Type
    id: string
    kitDirectory: string
    type: TypeInput
    constructor(
        id: string,
        kitDirectory: string,
        type: TypeInput = {
            name: 'Unsaved Type',
            variant: '',
            unit: 'm',
            representations: [],
            ports: [],
            qualities: []
        }
    ) {
        this.id = id
        this.kitDirectory = kitDirectory
        this.type = type
    }
    get name(): string {
        return this.type.name
    }
    get description(): string {
        return this.type.description ?? ''
    }
    get icon(): string {
        return this.type.icon ?? ''
    }
    toObject() {
        return {
            kind: this.kind,
            id: this.id,
            kitDirectory: this.kitDirectory,
            type: this.type
        }
    }
}

export interface ISelectionDesign {
    piecesIds: string[]
    connectionsPiecesIds: [string, string][] // [connectingPieceId, connectedPieceId]
}

export class DesignView implements IArtifactView {
    kind: ViewKind = ViewKind.Design
    id: string
    kitDirectory: string
    design: DesignInput
    selection: ISelectionDesign
    constructor(
        id: string,
        kitDirectory: string,
        design: DesignInput = {
            name: 'Unsaved Design',
            variant: '',
            unit: 'm',
            pieces: [],
            connections: []
        },
        selection: ISelectionDesign = { piecesIds: [], connectionsPiecesIds: [] }
    ) {
        this.id = id
        this.kitDirectory = kitDirectory
        this.design = design
        this.selection = selection
    }
    get name(): string {
        return this.design.name
    }
    get description(): string {
        return this.design.description ?? ''
    }
    get icon(): string {
        return this.design.icon ?? ''
    }
    toObject() {
        return {
            kind: this.kind,
            id: this.id,
            kitDirectory: this.kitDirectory,
            design: this.design,
            selection: this.selection
        }
    }
}

export const viewsSlice = createSlice({
    name: 'views',
    initialState: {
        views: []
    },
    reducers: {
        addView: {
            reducer: (
                state,
                action: PayloadAction<{
                    id: string
                    kitDirectory: string
                    viewKind: ViewKind
                    design?: DesignInput
                    type?: TypeInput
                    selection?: ISelectionDesign
                }>
            ) => {
                const { id, kitDirectory, viewKind, design, type, selection } = action.payload
                if (viewKind === ViewKind.Type) {
                    state.views.push({ kind: viewKind, id, kitDirectory, type })
                } else if (viewKind === ViewKind.Design) {
                    state.views.push({
                        kind: viewKind,
                        id,
                        kitDirectory,
                        design,
                        selection: selection ?? { piecesIds: [], connectionsPiecesIds: [] }
                    })
                }
            },
            prepare: (view: IArtifactView) => {
                return {
                    payload: {
                        viewKind: view.kind,
                        kitDirectory: view.kitDirectory,
                        id: view.id,
                        design:
                            view.kind === ViewKind.Design
                                ? (view as DesignView).design
                                : undefined,
                        type: view.kind === ViewKind.Type ? (view as TypeView).type : undefined,
                        selection:
                            view.kind === ViewKind.Design
                                ? (view as DesignView).selection
                                : undefined
                    }
                }
            }
        },
        removeView: (state, action: PayloadAction<string>) => {
            state.views = state.views.filter((view) => view.id !== action.payload)
        },
        updateDesign: (
            state,
            action: PayloadAction<{ id: string; design: DesignInput }>
        ) => {
            const designView = state.views.find(
                (view) => view.id === action.payload.id && view.kind === ViewKind.Design
            )
            if (designView) {
                designView.design = action.payload.design
            }
        },
        updateDesignSelection: {
            reducer: (
                state,
                action: PayloadAction<{
                    id: string
                    piecesIds: string[] | null | undefined,
                    connectionsPiecesIds: [string, string][] | null | undefined
                }>
            ) => {
                const designView = state.views.find(
                    (view) => view.id === action.payload.id && view.kind === ViewKind.Design
                )
                if (designView) {
                    if (action.payload.piecesIds) {
                        designView.selection.piecesIds = action.payload.piecesIds
                    }
                    if (action.payload.connectionsPiecesIds) {
                        designView.selection.connectionsPiecesIds =
                            action.payload.connectionsPiecesIds
                    }
                }
            },
            prepare: (
                id: string,
                piecesIds: string[] | null | undefined = null,
                connectionsPiecesIds: [string, string][] | null | undefined = null,
            ) => {
                return {
                    payload: {
                        id,
                        piecesIds: piecesIds,
                        connectionsPiecesIds: connectionsPiecesIds
                    }
                }
            }
        }
    }
})

export const { addView, removeView, updateDesign, updateDesignSelection } = viewsSlice.actions

export const selectViews = (state: { views: { views: IArtifactView[] } }): IArtifactView[] =>
    state.views.views

export const selectView = (
    state: { views: { views: IArtifactView[] } },
    id: string
): IArtifactView | undefined => state.views.views.find((view) => view.id === id)

export const selectDesignView = (
    state: { views: { views: IArtifactView[] } },
    id: string
): DesignView | undefined => {
    const view = state.views.views.find((view) => view.id === id)
    if (view && view.kind === ViewKind.Design) {
        return view as DesignView
    }
    return undefined
}

export const store = configureStore({
    reducer: {
        kits: kitsSlice.reducer,
        views: viewsSlice.reducer
    }
})

export type RootState = ReturnType<typeof store.getState>

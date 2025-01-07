import './App.scss'
import tailwindConfig from '../../../tailwind.config.js'
import React, {
    useState,
    forwardRef,
    Ref,
    useMemo,
    Fragment,
    Suspense,
    useEffect,
    SVGProps,
    useImperativeHandle,
    useRef,
    ReactNode,
    createContext,
    useContext
} from 'react'
import { createPortal } from 'react-dom'
import { KBarAnimator, KBarPortal, KBarPositioner, KBarProvider, KBarSearch } from 'kbar'
import { KBarResults, useMatches } from 'kbar'
import { ActionId, ActionImpl } from 'kbar'
import {
    Avatar,
    Breadcrumb,
    Button,
    Col,
    Collapse,
    ConfigProvider,
    Divider,
    Flex,
    Layout,
    Row,
    Select,
    Space,
    Tabs,
    message,
    MenuItem,
    Form,
    Radio,
    Input,
    FormProps,
    Tooltip,
    Modal,
    FloatButton,
    InputNumber,
    Slider,
    InputNumberProps
} from 'antd'
import enUS from 'antd/lib/calendar/locale/en_US'
import {
    Mesh,
    Line,
    Matrix4,
    MeshBasicMaterial,
    LineBasicMaterial,
    Color,
    Vector3,
    Object3DEventMap,
    Group
} from 'three'
import { Canvas, ThreeEvent, useLoader } from '@react-three/fiber'
import {
    OrbitControls,
    useGLTF,
    Select as ThreeSelect,
    GizmoHelper,
    GizmoViewport,
    TransformControls,
    Grid,
    Sphere,
    Line as DreiLine,
    // Box as DreiBox,
    Stage
} from '@react-three/drei'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import { INode, IEdge, IGraphInput, SelectionT, IPoint, GraphView } from 'react-digraph'
import SVG from 'react-inlinesvg'
import CloseSharpIcon from '@mui/icons-material/CloseSharp'
import MinimizeSharpIcon from '@mui/icons-material/MinimizeSharp'
import FullscreenSharpIcon from '@mui/icons-material/FullscreenSharp'
import FullscreenExitSharpIcon from '@mui/icons-material/FullscreenExitSharp'
import HomeSharpIcon from '@mui/icons-material/HomeSharp'
import FolderSharpIcon from '@mui/icons-material/FolderSharp'
import FileUploadSharpIcon from '@mui/icons-material/FileUploadSharp'
import OpenWithIcon from '@mui/icons-material/OpenWith'
import ThreeSixtyIcon from '@mui/icons-material/ThreeSixty'
import {
    DndContext,
    DragEndEvent,
    DragOverlay,
    DragStartEvent,
    useDraggable,
    useDroppable
} from '@dnd-kit/core'
import { nanoid } from '@reduxjs/toolkit'
import { useDispatch, useSelector } from 'react-redux'
import {
    Connection,
    ConnectionInput,
    Design,
    DesignIdInput,
    DesignInput,
    Piece,
    PieceInput,
    Port,
    PortInput,
    Representation,
    Type,
    TypeIdInput,
    TypeInput
} from './semio.d'
import {
    Hierarchy,
    TOLERANCE,
    radians,
    threeToSemioRotation,
    designToHierarchies,
    Point,
    Vector,
    Plane,
    Transform,
    semioToThreeRotation,
} from './semio'
import adjectives from './assets/adjectives'
import animals from './assets/animals'
import {
    RootState,
    addView,
    loadLocalKit,
    selectKit,
    selectTypes,
    selectViews,
    selectDesignView,
    DesignView,
    IArtifactView,
    ViewKind,
    selectView,
    TypeView,
    selectDesigns,
    selectType,
    updateDesign,
    updateDesignSelection,
    selectPorts,
    ISelectionDesign
} from './store'
import { ThemeConfig } from 'antd/lib'

// Copilot
// export type Maybe<T> = T | null;
// export type InputMaybe<T> = Maybe<T>;
// export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
// export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
// export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
// export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
// export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
// /** All built-in and custom scalars, mapped to their actual values */
// export type Scalars = {
//     ID: { input: string; output: string; }
//     String: { input: string; output: string; }
//     Boolean: { input: boolean; output: boolean; }
//     Int: { input: number; output: number; }
//     Float: { input: number; output: number; }
//     /**
//      * The `DateTime` scalar type represents a DateTime
//      * value as specified by
//      * [iso8601](https://en.wikipedia.org/wiki/ISO_8601).
//      */
//     DateTime: { input: any; output: any; }
// };

// export type Query = {
//     __typename?: 'Query';
//     loadLocalKit?: Maybe<LoadLocalKitResponse>;
//     designToSceneFromLocalKit?: Maybe<DesignToSceneFromLocalKitResponse>;
// };

// export type QueryLoadLocalKitArgs = {
//     directory: Scalars['String']['input'];
// };

// export type QueryDesignToSceneFromLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     designIdInput: DesignIdInput;
// };

// export type LoadLocalKitResponse = {
//     __typename?: 'LoadLocalKitResponse';
//     kit?: Maybe<Kit>;
//     error?: Maybe<LoadLocalKitError>;
// };

// /** 🗃️ A kit is a collection of types and designs. */
// export type Kit = {
//     __typename?: 'Kit';
//     name: Scalars['String']['output'];
//     description: Scalars['String']['output'];
//     icon: Scalars['String']['output'];
//     createdAt: Scalars['DateTime']['output'];
//     lastUpdateAt: Scalars['DateTime']['output'];
//     url: Scalars['String']['output'];
//     homepage: Scalars['String']['output'];
//     types: Array<Type>;
//     designs: Array<Design>;
// };

// /** 🧩 A type is a reusable element that can be connected with other types over ports. */
// export type Type = {
//     __typename?: 'Type';
//     name: Scalars['String']['output'];
//     description: Scalars['String']['output'];
//     icon: Scalars['String']['output'];
//     variant: Scalars['String']['output'];
//     unit: Scalars['String']['output'];
//     createdAt: Scalars['DateTime']['output'];
//     lastUpdateAt: Scalars['DateTime']['output'];
//     kit?: Maybe<Kit>;
//     representations: Array<Representation>;
//     ports: Array<Port>;
//     qualities: Array<Quality>;
//     pieces: Array<Piece>;
// };

// /** 💾 A representation is a link to a resource that describes a type for a certain level of detail and tags. */
// export type Representation = {
//     __typename?: 'Representation';
//     url: Scalars['String']['output'];
//     lod: Scalars['String']['output'];
//     type?: Maybe<Type>;
//     tags: Array<Scalars['String']['output']>;
// };

// /** 🔌 A port is a conceptual connection point (with a direction) of a type. */
// export type Port = {
//     __typename?: 'Port';
//     type?: Maybe<Type>;
//     locators: Array<Locator>;
//     connecteds: Array<Connection>;
//     connectings: Array<Connection>;
//     id: Scalars['String']['output'];
//     point: Point;
//     direction: Vector;
//     plane: Plane;
// };

// /** 🗺️ A locator is meta-data for grouping ports. */
// export type Locator = {
//     __typename?: 'Locator';
//     group: Scalars['String']['output'];
//     subgroup: Scalars['String']['output'];
//     port?: Maybe<Port>;
// };

// /** 🖇️ A bidirectional connection between two pieces of a design. */
// export type Connection = {
//     __typename?: 'Connection';
//     gap: Scalars['Float']['output'];
//     rotation: Scalars['Float']['output'];
//     design?: Maybe<Design>;
//     connected: Side;
//     connecting: Side;
// };

// /** 🏙️ A design is a collection of pieces that are connected. */
// export type Design = {
//     __typename?: 'Design';
//     name: Scalars['String']['output'];
//     description: Scalars['String']['output'];
//     icon: Scalars['String']['output'];
//     variant: Scalars['String']['output'];
//     unit: Scalars['String']['output'];
//     createdAt: Scalars['DateTime']['output'];
//     lastUpdateAt: Scalars['DateTime']['output'];
//     kit?: Maybe<Kit>;
//     pieces: Array<Piece>;
//     connections: Array<Connection>;
//     qualities: Array<Quality>;
// };

// /** ⭕ A piece is a 3d-instance of a type in a design. */
// export type Piece = {
//     __typename?: 'Piece';
//     type?: Maybe<Type>;
//     design?: Maybe<Design>;
//     connectings: Array<Connection>;
//     connecteds: Array<Connection>;
//     id: Scalars['String']['output'];
//     root?: Maybe<PieceRoot>;
//     diagram: PieceDiagram;
// };

// /** 🌱 The root indesign of a piece. */
// export type PieceRoot = {
//     __typename?: 'PieceRoot';
//     plane: Plane;
// };

// /** ◳ A plane is an origin (point) and an orientation (x-axis and y-axis). */
// export type Plane = {
//     __typename?: 'Plane';
//     origin: Point;
//     xAxis: Vector;
//     yAxis: Vector;
// };

// /** ✖️ A 3d-point (xyz) of floating point numbers. */
// export type Point = {
//     __typename?: 'Point';
//     x: Scalars['Float']['output'];
//     y: Scalars['Float']['output'];
//     z: Scalars['Float']['output'];
// };

// /** ➡️ A 3d-vector (xyz) of floating point numbers. */
// export type Vector = {
//     __typename?: 'Vector';
//     x: Scalars['Float']['output'];
//     y: Scalars['Float']['output'];
//     z: Scalars['Float']['output'];
// };

// /** ✏️ The diagram indesign of a piece. */
// export type PieceDiagram = {
//     __typename?: 'PieceDiagram';
//     point: DiagramPoint;
// };

// /** 📺 A 2d-point (xy) of integers in screen plane. */
// export type DiagramPoint = {
//     __typename?: 'DiagramPoint';
//     x: Scalars['Int']['output'];
//     y: Scalars['Int']['output'];
// };

// /** 📏 A quality is meta-data for decision making. */
// export type Quality = {
//     __typename?: 'Quality';
//     name: Scalars['String']['output'];
//     value: Scalars['String']['output'];
//     unit: Scalars['String']['output'];
//     definition: Scalars['String']['output'];
//     type?: Maybe<Type>;
//     design?: Maybe<Design>;
// };

// /** 🧱 A side of a piece in a connection. */
// export type Side = {
//     __typename?: 'Side';
//     piece: SidePiece;
// };

// /** ⭕ The piece indesign of a side. A piece is identified by an id (emtpy=default)). */
// export type SidePiece = {
//     __typename?: 'SidePiece';
//     id: Scalars['String']['output'];
//     type: SidePieceType;
// };

// /** 🧩 The type indesign of a piece of a side. */
// export type SidePieceType = {
//     __typename?: 'SidePieceType';
//     port?: Maybe<Port>;
// };

// export enum LoadLocalKitError {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToReadKit = 'NO_PERMISSION_TO_READ_KIT'
// }

// export type DesignToSceneFromLocalKitResponse = {
//     __typename?: 'DesignToSceneFromLocalKitResponse';
//     scene?: Maybe<Scene>;
//     error?: Maybe<DesignToSceneFromLocalKitResponseError>;
// };

// /** 🌆 A scene is a collection of objects. */
// export type Scene = {
//     __typename?: 'Scene';
//     objects: Array<Maybe<Object>>;
//     design?: Maybe<Design>;
// };

// /** 🗿 An object is a piece with a plane and a parent object (unless the piece is a root). */
// export type Object = {
//     __typename?: 'Object';
//     plane: Plane;
//     piece?: Maybe<Piece>;
//     parent?: Maybe<Object>;
// };

// export type DesignToSceneFromLocalKitResponseError = {
//     __typename?: 'DesignToSceneFromLocalKitResponseError';
//     code: DesignToSceneFromLocalKitResponseErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum DesignToSceneFromLocalKitResponseErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToReadKit = 'NO_PERMISSION_TO_READ_KIT',
//     DesignDoesNotExist = 'DESIGN_DOES_NOT_EXIST'
// }

// /** 🏙️ A design is identified by a name and optional variant. */
// export type DesignIdInput = {
//     name: Scalars['String']['input'];
//     variant?: InputMaybe<Scalars['String']['input']>;
// };

// export type Mutation = {
//     __typename?: 'Mutation';
//     createLocalKit?: Maybe<CreateLocalKitMutation>;
//     updateLocalKitMetadata?: Maybe<UpdateLocalKitMetadataMutation>;
//     deleteLocalKit?: Maybe<DeleteLocalKitMutation>;
//     addTypeToLocalKit?: Maybe<AddTypeToLocalKitMutation>;
//     removeTypeFromLocalKit?: Maybe<RemoveTypeFromLocalKitMutation>;
//     addDesignToLocalKit?: Maybe<AddDesignToLocalKitMutation>;
//     removeDesignFromLocalKit?: Maybe<RemoveDesignFromLocalKitMutation>;
// };

// export type MutationCreateLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     kitInput: KitInput;
// };

// export type MutationUpdateLocalKitMetadataArgs = {
//     directory: Scalars['String']['input'];
//     kitMetadataInput: KitMetadataInput;
// };

// export type MutationDeleteLocalKitArgs = {
//     directory: Scalars['String']['input'];
// };

// export type MutationAddTypeToLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     typeInput: TypeInput;
// };

// export type MutationRemoveTypeFromLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     typeId: TypeIdInput;
// };

// export type MutationAddDesignToLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     designInput: DesignInput;
// };

// export type MutationRemoveDesignFromLocalKitArgs = {
//     directory: Scalars['String']['input'];
//     designId: DesignIdInput;
// };

// export type CreateLocalKitMutation = {
//     __typename?: 'CreateLocalKitMutation';
//     kit?: Maybe<Kit>;
//     error?: Maybe<CreateLocalKitError>;
// };

// export type CreateLocalKitError = {
//     __typename?: 'CreateLocalKitError';
//     code: CreateLocalKitErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum CreateLocalKitErrorCode {
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryAlreadyContainsAKit = 'DIRECTORY_ALREADY_CONTAINS_A_KIT',
//     NoPermissionToCreateDirectory = 'NO_PERMISSION_TO_CREATE_DIRECTORY',
//     NoPermissionToCreateKit = 'NO_PERMISSION_TO_CREATE_KIT',
//     KitInputIsInvalid = 'KIT_INPUT_IS_INVALID'
// }

// /** 🗃️ A kit is a collection of types and designs. */
// export type KitInput = {
//     name: Scalars['String']['input'];
//     description?: InputMaybe<Scalars['String']['input']>;
//     icon?: InputMaybe<Scalars['String']['input']>;
//     url?: InputMaybe<Scalars['String']['input']>;
//     homepage?: InputMaybe<Scalars['String']['input']>;
//     types?: InputMaybe<Array<TypeInput>>;
//     designs?: InputMaybe<Array<DesignInput>>;
// };

// /** 🧩 A type is a reusable element that can be connected with other types over ports. */
// export type TypeInput = {
//     name: Scalars['String']['input'];
//     description?: InputMaybe<Scalars['String']['input']>;
//     icon?: InputMaybe<Scalars['String']['input']>;
//     variant?: InputMaybe<Scalars['String']['input']>;
//     unit: Scalars['String']['input'];
//     representations: Array<RepresentationInput>;
//     ports: Array<PortInput>;
//     qualities?: InputMaybe<Array<QualityInput>>;
// };

// /** 💾 A representation is a link to a resource that describes a type for a certain level of detail and tags. */
// export type RepresentationInput = {
//     url: Scalars['String']['input'];
//     lod?: InputMaybe<Scalars['String']['input']>;
//     tags?: InputMaybe<Array<Scalars['String']['input']>>;
// };

// /** 🔌 A port is a conceptual connection point (with a direction) of a type. */
// export type PortInput = {
//     id?: InputMaybe<Scalars['String']['input']>;
//     point: PointInput;
//     direction: VectorInput;
//     locators?: InputMaybe<Array<LocatorInput>>;
// };

// /** ✖️ A 3d-point (xyz) of floating point numbers. */
// export type PointInput = {
//     x?: InputMaybe<Scalars['Float']['input']>;
//     y?: InputMaybe<Scalars['Float']['input']>;
//     z?: InputMaybe<Scalars['Float']['input']>;
// };

// /** ➡️ A 3d-vector (xyz) of floating point numbers. */
// export type VectorInput = {
//     x?: InputMaybe<Scalars['Float']['input']>;
//     y?: InputMaybe<Scalars['Float']['input']>;
//     z?: InputMaybe<Scalars['Float']['input']>;
// };

// /** 🗺️ A locator is meta-data for grouping ports. */
// export type LocatorInput = {
//     group: Scalars['String']['input'];
//     subgroup?: InputMaybe<Scalars['String']['input']>;
// };

// /** 📏 A quality is meta-data for decision making. */
// export type QualityInput = {
//     name: Scalars['String']['input'];
//     value?: InputMaybe<Scalars['String']['input']>;
//     unit?: InputMaybe<Scalars['String']['input']>;
//     definition?: InputMaybe<Scalars['String']['input']>;
// };

// /** 🏙️ A design is a collection of pieces that are connected. */
// export type DesignInput = {
//     name: Scalars['String']['input'];
//     description?: InputMaybe<Scalars['String']['input']>;
//     icon?: InputMaybe<Scalars['String']['input']>;
//     variant?: InputMaybe<Scalars['String']['input']>;
//     unit: Scalars['String']['input'];
//     pieces: Array<PieceInput>;
//     connections: Array<ConnectionInput>;
//     qualities?: InputMaybe<Array<QualityInput>>;
// };

// /** ⭕ A piece is a 3d-instance of a type in a design. */
// export type PieceInput = {
//     id: Scalars['String']['input'];
//     type: TypeIdInput;
//     root?: InputMaybe<PieceRootInput>;
//     diagram: PieceDiagramInput;
// };

// /** 🧩 A type is identified by a name and variant (empty=default). */
// export type TypeIdInput = {
//     name: Scalars['String']['input'];
//     variant?: InputMaybe<Scalars['String']['input']>;
// };

// /** 🌱 The root indesign of a piece. */
// export type PieceRootInput = {
//     plane: PlaneInput;
// };

// /** ◳ A plane is an origin (point) and an orientation (x-axis and y-axis). */
// export type PlaneInput = {
//     origin: PointInput;
//     xAxis: VectorInput;
//     yAxis: VectorInput;
// };

// /** ✏️ The diagram indesign of a piece. */
// export type PieceDiagramInput = {
//     point: DiagramPointInput;
// };

// /** 📺 A 2d-point (xy) of integers in screen plane. */
// export type DiagramPointInput = {
//     x?: InputMaybe<Scalars['Int']['input']>;
//     y?: InputMaybe<Scalars['Int']['input']>;
// };

// /** 🖇️ A bidirectional connection between two pieces of a design. */
// export type ConnectionInput = {
//     connecting: SideInput;
//     connected: SideInput;
//     gap?: InputMaybe<Scalars['Float']['input']>;
//     rotation?: InputMaybe<Scalars['Float']['input']>;
// };

// /** 🧱 A side of a piece in a connection. */
// export type SideInput = {
//     piece: SidePieceInput;
// };

// /** ⭕ The piece indesign of a side. A piece is identified by an id (emtpy=default)). */
// export type SidePieceInput = {
//     id: Scalars['String']['input'];
//     type?: InputMaybe<SidePieceTypeInput>;
// };

// /** 🧩 The type indesign of a piece of a side. */
// export type SidePieceTypeInput = {
//     port?: InputMaybe<PortIdInput>;
// };

// /** 🔌 A port is identified by an id (emtpy=default)). */
// export type PortIdInput = {
//     id?: InputMaybe<Scalars['String']['input']>;
// };

// export type UpdateLocalKitMetadataMutation = {
//     __typename?: 'UpdateLocalKitMetadataMutation';
//     kit?: Maybe<Kit>;
//     error?: Maybe<UpdateLocalKitMetadataError>;
// };

// export type UpdateLocalKitMetadataError = {
//     __typename?: 'UpdateLocalKitMetadataError';
//     code: UpdateLocalKitMetadataErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum UpdateLocalKitMetadataErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToUpdateKit = 'NO_PERMISSION_TO_UPDATE_KIT',
//     KitMetadataIsInvalid = 'KIT_METADATA_IS_INVALID'
// }

// /** 🗃️ Meta-data of a kit. */
// export type KitMetadataInput = {
//     name?: InputMaybe<Scalars['String']['input']>;
//     description?: InputMaybe<Scalars['String']['input']>;
//     icon?: InputMaybe<Scalars['String']['input']>;
//     url?: InputMaybe<Scalars['String']['input']>;
//     homepage?: InputMaybe<Scalars['String']['input']>;
// };

// export type DeleteLocalKitMutation = {
//     __typename?: 'DeleteLocalKitMutation';
//     error?: Maybe<DeleteLocalKitError>;
// };

// export enum DeleteLocalKitError {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToDeleteKit = 'NO_PERMISSION_TO_DELETE_KIT'
// }

// export type AddTypeToLocalKitMutation = {
//     __typename?: 'AddTypeToLocalKitMutation';
//     type?: Maybe<Type>;
//     error?: Maybe<AddTypeToLocalKitError>;
// };

// export type AddTypeToLocalKitError = {
//     __typename?: 'AddTypeToLocalKitError';
//     code: AddTypeToLocalKitErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum AddTypeToLocalKitErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToModifyKit = 'NO_PERMISSION_TO_MODIFY_KIT',
//     TypeInputIsInvalid = 'TYPE_INPUT_IS_INVALID'
// }

// export type RemoveTypeFromLocalKitMutation = {
//     __typename?: 'RemoveTypeFromLocalKitMutation';
//     error?: Maybe<RemoveTypeFromLocalKitError>;
// };

// export type RemoveTypeFromLocalKitError = {
//     __typename?: 'RemoveTypeFromLocalKitError';
//     code: RemoveTypeFromLocalKitErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum RemoveTypeFromLocalKitErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToModifyKit = 'NO_PERMISSION_TO_MODIFY_KIT',
//     TypeDoesNotExist = 'TYPE_DOES_NOT_EXIST',
//     DesignDependsOnType = 'DESIGN_DEPENDS_ON_TYPE'
// }

// export type AddDesignToLocalKitMutation = {
//     __typename?: 'AddDesignToLocalKitMutation';
//     design?: Maybe<Design>;
//     error?: Maybe<AddDesignToLocalKitError>;
// };

// export type AddDesignToLocalKitError = {
//     __typename?: 'AddDesignToLocalKitError';
//     code: AddDesignToLocalKitErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum AddDesignToLocalKitErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToModifyKit = 'NO_PERMISSION_TO_MODIFY_KIT',
//     DesignInputIsInvalid = 'DESIGN_INPUT_IS_INVALID'
// }

// export type RemoveDesignFromLocalKitMutation = {
//     __typename?: 'RemoveDesignFromLocalKitMutation';
//     error?: Maybe<RemoveDesignFromLocalKitError>;
// };

// export type RemoveDesignFromLocalKitError = {
//     __typename?: 'RemoveDesignFromLocalKitError';
//     code: RemoveDesignFromLocalKitErrorCode;
//     message?: Maybe<Scalars['String']['output']>;
// };

// export enum RemoveDesignFromLocalKitErrorCode {
//     DirectoryDoesNotExist = 'DIRECTORY_DOES_NOT_EXIST',
//     DirectoryIsNotADirectory = 'DIRECTORY_IS_NOT_A_DIRECTORY',
//     DirectoryHasNoKit = 'DIRECTORY_HAS_NO_KIT',
//     NoPermissionToModifyKit = 'NO_PERMISSION_TO_MODIFY_KIT',
//     DesignDoesNotExist = 'DESIGN_DOES_NOT_EXIST'
// }

const { Header, Content, Footer, Sider } = Layout

const {
    theme: { colors }
} = tailwindConfig

const sketchpadTheme = {
    // algorithm: [theme.darkAlgorithm],
    token: {
        // primary
        colorPrimary: colors.light,
        colorPrimaryBg: colors.light,
        colorPrimaryBgHover: colors.light,
        colorPrimaryBorder: colors.light,
        colorPrimaryBorderHover: colors.light,
        colorPrimaryHover: colors.light, // e.g. hover primary button
        colorPrimaryActive: colors.light,
        colorPrimaryText: colors.light,
        colorPrimaryTextHover: colors.light,
        colorPrimaryTextActive: colors.light,
        // text
        colorTextBase: colors.light,
        colorText: colors.light, // e.g. title of collapse, leaf of breadcrumb
        colorTextSecondary: colors.lightGrey,
        colorTextTertiary: colors.lightGrey, // e.g. x on close button of tab
        colorTextQuaternary: colors.lightGrey, // e.g. placeholder text
        colorTextDescription: colors.light,
        colorTextHeading: colors.light,
        colorTextLabel: colors.light,
        colorTextLightSolid: colors.light,
        colorTextPlaceholder: colors.light,
        colorTextDisabled: colors.light,
        // border
        colorBorder: colors.light,
        colorBorderSecondary: colors.light,
        // fill
        colorFill: colors.light,
        colorFillSecondary: colors.light,
        colorFillTertiary: colors.light,
        colorFillQuaternary: colors.darkGrey, // e.g. background of collapse title
        // background
        colorBgContainer: colors.darkGrey, // e.g. active tab, collapse content box
        colorBgElevated: colors.grey, // e.g. background selected menu
        colorBgLayout: colors.light,
        colorBgSpotlight: colors.grey, // e.g background of tooltip
        colorBgMask: colors.light,
        colorBgTextActive: colors.light,
        colorBgBase: colors.light,
        controlItemBgHover: colors.light,
        // special colors
        colorError: colors.danger,
        colorWarning: colors.warning,
        colorInfo: colors.info,
        colorSuccess: colors.success,
        fontFamily: 'Anta, sans-serif',
        boxShadow: 'none',
        boxShadowSecondary: 'none',
        boxShadowTertiary: 'none',
        wireframe: false,
        borderRadius: 0,
        lineType: 'none',
        lineWidth: 0
        // TODO: Fast motion without modal freeze
        // motionUnit: 0.001, // Makes modal freeze somehow and overwriting it on Modal doesn't work.
    },
    components: {
        Button: {
            borderColorDisabled: colors.light,
            dangerColor: colors.light,
            defaultActiveBg: colors.light,
            defaultActiveBorderColor: colors.light,
            defaultActiveColor: colors.light,
            defaultBg: colors.grey, // e.g. background of window control buttons
            defaultBorderColor: colors.light,
            defaultColor: colors.lightGrey, // e.g. normal state of buttons
            defaultGhostBorderColor: colors.light,
            defaultGhostColor: colors.light,
            defaultHoverBg: colors.darkGrey, // e.g. hover over window control buttons,
            defaultHoverBorderColor: colors.light,
            defaultHoverColor: colors.light,
            ghostBg: colors.light,
            groupBorderColor: colors.light,
            linkHoverBg: colors.light,
            primaryColor: colors.light,
            textHoverBg: colors.light
        },
        FloatButton: {},
        Layout: {
            bodyBg: colors.dark, //
            footerBg: colors.grey, //
            headerBg: colors.grey, // e.g. space between tabs and content
            headerColor: colors.light,
            lightSiderBg: colors.light,
            lightTriggerBg: colors.light,
            lightTriggerColor: colors.light,
            siderBg: colors.darkGrey, //
            triggerBg: colors.light,
            triggerColor: colors.light,
            headerPadding: '0px 0px'
        },
        Tabs: {
            cardBg: colors.grey, // background of unselected tabs
            inkBarColor: colors.light,
            itemActiveColor: colors.light,
            itemColor: colors.lightGrey, // text and fill of unselected tabs
            itemHoverColor: colors.light,
            itemSelectedColor: colors.light,
            cardGutter: 0,
            cardHeight: 38,
            cardPadding: '0 16px',
            verticalItemMargin: '0'
        },
        Divider: {
            lineWidth: 0.25,
            verticalMarginInline: 0
        },
        Avatar: {
            groupBorderColor: colors.light
        },
        Collapse: {
            headerBg: colors.darkGrey, //
            headerPadding: '0 0px',
            contentBg: colors.darkGrey, //
            contentPadding: '0 0px'
        },
        Select: {
            clearBg: colors.lightGrey, //
            multipleItemBg: colors.darkGrey, //
            optionActiveBg: colors.darkGrey, //
            optionSelectedBg: colors.darkGrey, //
            optionSelectedColor: colors.light,
            selectorBg: colors.darkGrey //
        },
        Form: {
            labelColor: colors.lightGrey, // e.g. text of label
            labelRequiredMarkColor: colors.light
        },
        Radio: {
            buttonBg: colors.grey, //
            buttonCheckedBg: colors.lightGrey, //
            buttonCheckedBgDisabled: colors.light,
            buttonCheckedColorDisabled: colors.light,
            buttonColor: colors.lightGrey, // e.g. text of radio
            buttonSolidCheckedActiveBg: colors.light,
            buttonSolidCheckedColor: colors.light,
            buttonSolidCheckedHoverBg: colors.light,
            dotColorDisabled: colors.light
        },
        Tooltip: {}
    }
} as ThemeConfig

const Label = ({
    children,
    className
}: {
    children: ReactNode
    className?: string
}): JSX.Element => {
    return <span className={`text-lightGrey ${className}`}>{children}</span>
}

class SeededRandom {
    private seed: number

    constructor(seed: number) {
        this.seed = seed % 2147483647
        if (this.seed <= 0) this.seed += 2147483646
    }

    // Returns a pseudo-random number between 1 and 2^31 - 2
    next(): number {
        return (this.seed = (this.seed * 16807) % 2147483647)
    }

    // Returns a pseudo-random number between 0 (inclusive) and 1 (exclusive)
    nextFloat(): number {
        return (this.next() - 1) / 2147483646
    }

    // Returns a pseudo-random number between 0 (inclusive) and max (exclusive)
    nextInt(max: number): number {
        return Math.floor(this.nextFloat() * max)
    }
}

class Generator {
    public static generateRandomId(seed?: number | undefined): string {
        if (seed === undefined) {
            seed = Math.floor(Math.random() * 1000000)
        }
        const random = new SeededRandom(seed)

        let adjective = adjectives[random.nextInt(adjectives.length)]
        let animal = animals[random.nextInt(animals.length)]
        const number = random.nextInt(1000)

        adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1)
        animal = animal.charAt(0).toUpperCase() + animal.slice(1)

        return `${adjective}${animal}${number}`
    }
}

const SEPARATOR = '::'
const kindNameAndVariantToString = (kind: string, name: string, variant?: string): string => {
    return `${kind}${SEPARATOR}${name}${SEPARATOR}${variant}`
}
const typeToIdString = (type: Type): string => {
    return kindNameAndVariantToString('type', type.name, type.variant)
}
const designToIdString = (design: Design): string => {
    return kindNameAndVariantToString('design', design.name, design.variant)
}
const typeToHumanString = (type: Type | TypeInput | TypeIdInput): string => {
    const { name, variant } = type
    return `${name}${variant ? ` (${variant})` : ''}`
}
const designToHumanString = (design: Design | DesignInput | DesignIdInput): string => {
    const { name, variant } = design
    return `${name}${variant ? ` (${variant})` : ''}`
}

function tinyKeyStringToHuman(string: string): string {
    return string
        .split('+')
        .map((key) => {
            if (key === '$mod') return 'Ctrl'
            if (key === 'Shift') return '⇧'
            return key
        })
        .join(' + ')
}

const ResultItem = forwardRef(
    (
        {
            action,
            active,
            currentRootActionId
        }: {
            action: ActionImpl
            active: boolean
            currentRootActionId: ActionId
        },
        ref: Ref<HTMLDivElement>
    ): JSX.Element => {
        const ancestors = useMemo(() => {
            if (!currentRootActionId) return action.ancestors
            const index = action.ancestors.findIndex(
                (ancestor) => ancestor.id === currentRootActionId
            )
            // +1 removes the currentRootAction; e.g.
            // if we are on the 'Set theme' parent action,
            // the UI should not display 'Set theme… > Dark'
            // but rather just 'Dark'
            return action.ancestors.slice(index + 1)
        }, [action.ancestors, currentRootActionId])

        return (
            <div
                ref={ref}
                className={`flex justify-between px-4 rounded-md  ${active ? 'bg-primary text-dark' : 'bg-dark bg-opacity-50 text-light'}`}
            >
                <div className="description">
                    {action.icon && action.icon}
                    <div>
                        <div>
                            {ancestors.length > 0 &&
                                ancestors.map((ancestor) => (
                                    <Fragment key={ancestor.id}>
                                        <span>{ancestor.name}</span>
                                        <span>&rsaquo;</span>
                                    </Fragment>
                                ))}
                            <span>{action.name}</span>
                        </div>
                        {action.subtitle && <span>{action.subtitle}</span>}
                    </div>
                </div>
                {action.shortcut?.length ? (
                    <div className="shortcut">
                        {action.shortcut.map((sc) => (
                            <kbd key={sc}>{tinyKeyStringToHuman(sc)}</kbd>
                        ))}
                    </div>
                ) : null}
            </div>
        )
    }
)

ResultItem.displayName = 'ResultItem'

interface RenderResultsProps {
    className?: string
}

function RenderResults({ className }: RenderResultsProps): JSX.Element {
    const { results, rootActionId } = useMatches()

    return (
        <KBarResults
            items={results}
            onRender={({ item, active }) =>
                typeof item === 'string' ? (
                    <div className={active ? `${className} active` : className}>{item}</div>
                ) : (
                    <ResultItem action={item} active={active} currentRootActionId={rootActionId} />
                )
            }
        />
    )
}

function CommandBar(): JSX.Element {
    return (
        <KBarPortal>
            <KBarPositioner className="backdrop-blur-sm">
                <KBarAnimator className="w-2/3">
                    <KBarSearch className="w-full bg-light border-none p-4 rounded-2xl placeholder:text-dark focus:bg-primary focus:outline-none focus:placeholder:text-light selection:bg-secondary" />
                    <RenderResults className=" bg-light bg-opacity-50 rounded-md px-2 py-1 box-content" />
                </KBarAnimator>
            </KBarPositioner>
        </KBarPortal>
    )
}

enum IconKind {
    Text,
    Svg,
    Image
}

function getIconData(dataUrl): [string, IconKind] {
    const svgStart = 'data:image/svg+xml;base64,'
    const pngStart = 'data:image/png;base64,'
    const jpegStart = 'data:image/jpeg;base64,'
    let kind
    let data
    if (dataUrl.startsWith(svgStart)) {
        kind = IconKind.Svg
        data = atob(dataUrl.substring(svgStart.length))
    } else if (dataUrl.startsWith(pngStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(pngStart.length));
        data = dataUrl
    } else if (dataUrl.startsWith(jpegStart)) {
        kind = IconKind.Image
        // data = atob(dataUrl.substring(jpegStart.length));
        data = dataUrl
    } else {
        kind = IconKind.Text
        data = dataUrl
    }
    return [data, kind]
}

const turnBlackAndWhiteSvgSemiotic = (svgString: string, dark = false): string => {
    if (dark)
        return svgString
            .replace(/#000000/g, colors.light)
            .replace(/#000/g, colors.light)
            .replace(/black/g, colors.light)
            .replace(/#FFFFFF/g, colors.dark)
            .replace(/#FFF/g, colors.dark)
            .replace(/white/g, colors.dark)
    return svgString
        .replace(/#000000/g, colors.dark)
        .replace(/#000/g, colors.dark)
        .replace(/black/g, colors.dark)
        .replace(/#FFFFFF/g, colors.light)
        .replace(/#FFF/g, colors.light)
        .replace(/white/g, colors.light)
}

interface ArtifactAvatarProps {
    icon: string
    description?: ReactNode
    isSelected?: boolean
    draggableId?: string
}

const ArtifactAvatar = ({
    icon,
    description,
    isSelected,
    draggableId
}: ArtifactAvatarProps): JSX.Element => {
    const [data, kind] = getIconData(icon)
    const draggableProps = draggableId
        ? (() => {
            const { attributes, listeners, setNodeRef } = useDraggable({
                id: draggableId
            })

            return {
                ref: setNodeRef,
                ...listeners,
                ...attributes
            }
        })()
        : {}

    switch (kind) {
        case IconKind.Svg:
            return (
                <Tooltip placement="right" title={description}>
                    <Avatar
                        className={`font-sans cursor-pointer ${isSelected ? 'bg-primary text-light' : 'bg-light text-darkGrey'}`}
                        size={38}
                        {...draggableProps}
                    >
                        <SVG
                            src={turnBlackAndWhiteSvgSemiotic(data, isSelected)}
                            width="32"
                            height="32"
                        />
                    </Avatar>
                </Tooltip>
            )
        case IconKind.Image:
            return (
                <Tooltip placement="right" title={description}>
                    <Avatar
                        className={`cursor-pointer ${isSelected ? 'bg-primary opacity-50' : 'bg-light opacity-100'}`}
                        src={data}
                        size={38}
                        {...draggableProps}
                    ></Avatar>
                </Tooltip>
            )
        case IconKind.Text:
            return (
                <Tooltip placement="right" title={description}>
                    <Avatar
                        className={`font-sans cursor-pointer ${isSelected ? 'bg-primary text-light' : 'bg-light text-darkGrey'}`}
                        size={38}
                        {...draggableProps}
                    >
                        {data}
                    </Avatar>
                </Tooltip>
            )
    }
}

const getGroupNameFromClickEventGroupObject = (o: any): string => {
    if (o.name !== '') return o.name

    const childGroupWithId = o.children.find((element) => {
        if (element?.isGroup !== true) return false
        const childGroupId = getGroupNameFromClickEventGroupObject(element)
        return childGroupId !== ''
    })

    return childGroupWithId ? getGroupNameFromClickEventGroupObject(childGroupWithId) : ''
}

const Gizmo = (): JSX.Element => {
    return (
        <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
            <GizmoViewport
                labels={['X', 'Z', '-Y']}
                axisColors={[colors.primary, colors.tertiary, colors.secondary]}
            // font='Anta'
            />
        </GizmoHelper>
    )
}

interface PointThreeProps {
    point: Point
    color: string
    size: number
}

const PointThree = ({ point, color, size }: PointThreeProps): JSX.Element => {
    return (
        <Sphere
            args={[size / 2]}
            position={point.toArray()}
            material={new MeshBasicMaterial({ color: new Color(color) })}
        />
    )
}

interface SemioCanvasProps {
    children: React.ReactNode
    onPointerMissed?: (event: MouseEvent) => void
}
// TODO: Refactor to extend Canvas: https://docs.pmnd.rs/react-three-fiber/tutorials/typescript#extending-threeelements
const SemioCanvas = ({ children, onPointerMissed }: SemioCanvasProps): JSX.Element => {
    return (
        <Canvas
            // orthographic={true}
            shadows={false}
            onPointerMissed={onPointerMissed}
        >
            <OrbitControls makeDefault />
            <Gizmo />
            <Grid infiniteGrid={true} sectionColor={colors.lightGrey} />
            <Stage center={{ disable: true }} environment={null}>
                <Suspense fallback={null}>
                    <group matrix={semioToThreeRotation().toArray()} matrixAutoUpdate={false}>
                        {/* <PointThree point={new Point(0, 0, 0)} color={colors.light} size={0.1} /> */}
                        {children}
                    </group>
                </Suspense>
            </Stage>
        </Canvas>
    )
}

// interface PlaneThreeProps {
//     plane: Plane
//     lineWidth?: number
//     onSelect: (event: ThreeEvent<MouseEvent>) => void
// }

// const PlaneThree = ({ plane, lineWidth, onSelect }: PlaneThreeProps) => {
//     if (!lineWidth) lineWidth = 1
//     const groupRef = useRef();
//     useEffect(() => {
//         if (groupRef.current) {
//             const transform = planeToTransform(plane)
//             groupRef.current.applyMatrix4(transform)
//         }
//     }, [])
//     return (
//         <group name='plane' ref={groupRef}>
//             <DreiLine
//                 // name='x-axis'
//                 points={[[0, 0, 0], [1, 0, 0]]}
//                 color={colors.primary}
//                 lineWidth={lineWidth * 2}
//             />
//             <DreiLine
//                 // name='y-axis'
//                 points={[[0, 0, 0], [0, 0, -1]]}
//                 color={colors.secondary}
//                 lineWidth={lineWidth * 2}
//             />
//             <DreiLine
//                 // name='z-axis'
//                 points={[[0, 0, 0], [0, 1, 0]]}
//                 color={colors.tertiary}
//                 lineWidth={lineWidth * 2}
//             />
//             <DreiLine
//                 points={[[-1, 0, -1], [1, 0, -1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, -1], [-1, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, -0.666667], [1, 0, -0.666667]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, -0.333333], [1, 0, -0.333333]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, 0.333333], [1, 0, 0.333333]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, 0.666667], [1, 0, 0.666667]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, 1], [1, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-0.666667, 0, -1], [-0.666667, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-0.333333, 0, -1], [-0.333333, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[0.333333, 0, -1], [0.333333, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[0.666667, 0, -1], [0.666667, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[1, 0, -1], [1, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[0, 0, 0], [0, 0, 1]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiLine
//                 points={[[-1, 0, 0], [0, 0, 0]]} color={colors.grey}
//                 lineWidth={lineWidth}
//             />
//             <DreiBox
//                 args={[2.0, 1.0, 2.0]}
//                 position={[0, 0.5, 0]}
//                 material={new MeshBasicMaterial({ transparent: true, opacity: 0 })}
//                 onClick={(event) => {
//                     onSelect(event)
//                     event.stopPropagation()
//                 }}
//             />
//         </group>
//     )
// }

interface PortThreeProps {
    port: Port | PortInput
    selected: boolean
    onClick: (event: ThreeEvent<MouseEvent>) => void
}

const PortThree = ({ port, selected, onClick }: PortThreeProps): JSX.Element => {
    const point = Point.parse(port.point)
    const direction = Vector.parse(port.direction)

    return (
        <group name="port" onClick={onClick}>
            <PointThree
                point={point}
                color={selected ? colors.primary : colors.grey}
                size={selected ? 0.2 : 0.1}
            />
            <DreiLine
                points={[point.toArray(), new Vector3().addVectors(point, direction).toArray()]}
                color={colors.light}
                lineWidth={selected ? 3 : 1}
            />
        </group>
    )
}

interface RepresentationThreeProps {
    representation: Representation
    color?: string
    id?: string
    transform?: Transform // in semio coordinates
}

const RepresentationThree = ({
    representation,
    id,
    color,
    transform
}: RepresentationThreeProps): JSX.Element => {
    const { blobUrls } = useContext(EditorContext)
    const representationThreeScene = useMemo(() => {
        const clone = useLoader(GLTFLoader, blobUrls[representation.url]).scene.clone()
        clone.applyMatrix4(threeToSemioRotation())
        return clone
    }, [representation.url])
    useMemo(() => {
        representationThreeScene.traverse((object) => {
            if (object instanceof Mesh) {
                const meshColor = color ? new Color(color) : new Color(colors.lightGrey)
                object.material = new MeshBasicMaterial({ color: meshColor })
            }
            if (object instanceof Line) {
                const lineColor = new Color(colors.dark)
                object.material = new LineBasicMaterial({ color: lineColor })
            }
        })
    }, [color])
    useMemo(() => {
        if (transform) {
            representationThreeScene.matrix = transform
            representationThreeScene.matrixAutoUpdate = false
        }
    }, [transform])
    representationThreeScene.name = id
    return <primitive object={representationThreeScene} />
}

RepresentationThree.displayName = 'Representation'

interface PortSelectorProps {
    type: Type | TypeInput
    selectedPortId: string
    onSelect: (portId: string) => void
}

const PortSelector = ({ type, selectedPortId, onSelect }: PortSelectorProps): JSX.Element => {
    const ports = type.ports

    return (
        <div className="w-[350px] h-[350px]">
            <SemioCanvas>
                {/* TODO: Proper filtering */}
                <RepresentationThree
                    id={type.id}
                    representation={type.representations.find((r) => r.url.endsWith('.glb'))}
                />
                {ports.map((port) => (
                    <PortThree
                        key={port.id}
                        port={port}
                        selected={selectedPortId === port.id}
                        onClick={(e) => {
                            onSelect(port.id)
                            e.stopPropagation()
                        }}
                    />
                ))}
            </SemioCanvas>
        </div>
    )
}

interface ConnectionPreview {
    connectedType: Type | TypeInput
    connectingType: Type | TypeInput
    connection: Connection | ConnectionInput
}

const ConnectionPreview = ({
    connectedType,
    connectingType,
    connection
}: ConnectionPreview): JSX.Element => {
    const parentPort = connectedType.ports.find(
        (port) => port.id === connection.connected.piece.type?.port?.id
    )
    const childPort = connectingType.ports.find(
        (port) => port.id === connection.connecting.piece.type?.port?.id
    )
    const parentDirection = Vector.parse(parentPort.direction)
    const childDirection = Vector.parse(childPort.direction)
    const parentPoint = Point.parse(parentPort.point)
    const childPoint = Point.parse(childPort.point)
    const orient = Transform.fromDirections(childDirection.negate(), parentDirection)
    let rotation = orient
    if (connection.rotation !== 0) {
        const rotate = Transform.fromAngle(parentDirection, connection.rotation)
        rotation = rotate.after(orient)
    }
    const centerChild = childPoint.toVector().negate().toTransform()
    const moveToParent = parentPoint.toVector().toTransform()
    let transform = new Transform()
    transform = rotation.after(centerChild)
    if (connection.gap !== 0) {
        const gap = parentDirection.clone().multiplyScalar(connection.gap).toTransform()
        transform = gap.after(transform)
    }
    transform = moveToParent.after(transform)
    return (
        <div className="w-[700px] h-[700px]">
            <SemioCanvas>
                <RepresentationThree
                    id="connected"
                    // TODO: Proper filtering
                    representation={connectedType.representations.find((r) =>
                        r.url.endsWith('.glb')
                    )}
                />
                <RepresentationThree
                    id="connecting"
                    // TODO: Proper filtering
                    representation={connectingType.representations.find((r) =>
                        r.url.endsWith('.glb')
                    )}
                    transform={transform}
                />
            </SemioCanvas>
        </div>
    )
}

const findDefaultOrFirstPort = (type: Type | TypeInput): Port | PortInput => {
    return type.ports.find((port) => port.id === '') ?? type.ports[0]
}

interface ConnectionBuilderProps {
    connectingType: Type | TypeInput
    connectedType: Type | TypeInput
    onConnectionChange: (connection: ConnectionInput) => void
}

const ConnectionBuilder = ({
    connectingType,
    connectedType,
    onConnectionChange
}: ConnectionBuilderProps): JSX.Element => {
    const [connectingPortId, setConnectingPortId] = useState(
        findDefaultOrFirstPort(connectingType).id
    )
    const [connectedPortId, setConnectedPortId] = useState(findDefaultOrFirstPort(connectedType).id)
    const [gap, setGap] = useState(0)
    const [rotation, setRotation] = useState(0)

    const onRotationChange: InputNumberProps['onChange'] = (newValue) => {
        setRotation(newValue as number)
    }

    const connection = {
        connecting: {
            piece: {
                type: {
                    port: {
                        id: connectingPortId
                    }
                }
            }
        },
        connected: {
            piece: {
                type: {
                    port: {
                        id: connectedPortId
                    }
                }
            }
        },
        gap,
        rotation
    } as ConnectionInput

    useEffect(() => {
        onConnectionChange(connection)
    }, [connectingType, connectedType, connectingPortId, connectedPortId])

    return (
        <Flex>
            <Flex vertical>
                <PortSelector
                    type={connectingType}
                    onSelect={setConnectingPortId}
                    selectedPortId={connectingPortId}
                />
                <Divider className="m-0" />
                <PortSelector
                    type={connectedType}
                    onSelect={setConnectedPortId}
                    selectedPortId={connectedPortId}
                />
                <Flex>
                    <Label>Gap</Label>
                    <InputNumber value={gap} onChange={(value) => setGap(value)} />
                </Flex>
                <Row>
                    <Col span={12}>
                        <Slider
                            min={-360}
                            max={360}
                            onChange={onRotationChange}
                            value={typeof rotation === 'number' ? rotation : 0}
                        />
                    </Col>
                    <Col span={4}>
                        <InputNumber
                            min={-360}
                            max={360}
                            style={{ margin: '0 16px' }}
                            value={rotation}
                            onChange={onRotationChange}
                        />
                    </Col>
                </Row>
            </Flex>
            <Divider className="h-auto" type="vertical" />
            <Flex vertical>
                <ConnectionPreview
                    connectingType={connectingType}
                    connectedType={connectedType}
                    connection={connection}
                />
            </Flex>
        </Flex>
    )
}

const GraphConfig = {
    NodeTypes: {
        piece: {
            typeText: '',
            shapeId: '#piece',
            shape: (
                <symbol
                    className="piece"
                    viewBox="0 0 50 50"
                    height="40"
                    width="40"
                    id="piece"
                    key="0"
                >
                    <circle cx="25" cy="25" r="24"></circle>
                </symbol>
            )
        }
    },
    NodeSubtypes: {},
    EdgeTypes: {
        connection: {
            shapeId: '#connection',
            shape: (
                <symbol viewBox="0 0 50 50" id="connection" key="0">
                    {/* <circle cx='25' cy='25' r='8' fill='currentColor'> </circle> */}
                </symbol>
            )
        }
    }
}

interface IPieceNode extends INode {
    piece: Piece | PieceInput
}

interface IConnectionEdge extends IEdge {
    connection: Connection | ConnectionInput
}

export interface IDraft extends IGraphInput {
    name?: string
    description?: string
    icon?: string
    nodes: IPieceNode[]
    edges: IConnectionEdge[]
}

const NODE_KEY = 'id' // Allows D3 to correctly update DOM

const transformPieceToNode = (piece: Piece | PieceInput): IPieceNode => {
    return {
        id: piece.id,
        title: '',
        type: 'piece',
        x: Math.round(piece.diagram.point.x),
        y: Math.round(piece.diagram.point.y),
        piece
    }
}
const transformConnectionToEdge = (connection: Connection | ConnectionInput): IConnectionEdge => {
    const sourceLabel =
        (connection.connected.piece.type?.port?.id ?? '') === ''
            ? '""'
            : connection.connected.piece.type?.port?.id
    const targetLabel =
        (connection.connecting.piece.type?.port?.id ?? '') === ''
            ? '""'
            : connection.connecting.piece.type?.port?.id
    return {
        source: connection.connected.piece.id,
        target: connection.connecting.piece.id,
        // label_from: sourceLabel,
        // label_to: targetLabel,
        handleTooltipText: sourceLabel + ' - ' + targetLabel,
        type: 'connection',
        connection
    }
}

const transformDesignToGraph = (design: Design | DesignInput): IDraft => {
    const nodes = design.pieces.map((piece) => transformPieceToNode(piece))

    const edges = design.connections.map((connection) => transformConnectionToEdge(connection))

    return {
        nodes,
        edges
    }
}

const transformSelectionToGraph = (
    design: Design | DesignInput,
    selection: ISelectionDesign
): SelectionT => {
    const nodes = new Map<string, INode>()
    const edges = new Map<string, IEdge>()
    selection.piecesIds.forEach((pieceId) => {
        const piece = design.pieces.find((p) => p.id === pieceId)
        if (piece) {
            nodes.set(piece.id, transformPieceToNode(piece))
        }
    })
    selection.connectionsPiecesIds.forEach(([sourceId, targetId]) => {
        const connection = design.connections.find(
            (a) => a.connecting.piece.id === sourceId && a.connected.piece.id === targetId
        )
        if (connection) {
            edges.set(`${sourceId}_${targetId}`, transformConnectionToEdge(connection))
        }
    })
    return { nodes, edges }
}

interface DiagramEditorProps {
    piece: PieceInput
    onPieceEdit: (piece: PieceInput) => Promise<PieceInput>
    onConnectionEdit: (connection: ConnectionInput) => ConnectionInput
    className?: string
}

const DiagramEditor = forwardRef((props: DiagramEditorProps, ref) => {
    const { designViewId, kitDirectory } = useContext(EditorContext)
    const dispatch = useDispatch()
    const types = useSelector((state: RootState) => selectTypes(state, kitDirectory))
    const designView = useSelector((state: RootState) => selectDesignView(state, designViewId))
    if (!designView) return null
    const designRef = useRef(designView.design)
    useEffect(() => {
        designRef.current = designView.design
    }, [designView.design])

    const graph = useMemo(() => transformDesignToGraph(designView.design), [designView.design])
    const nodes = graph.nodes
    const edges = graph.edges

    const selected = useMemo(
        () => transformSelectionToGraph(designView.design, designView.selection),
        [designView.design, designView.selection]
    )

    const graphViewRef = useRef(null)

    const { isOver, setNodeRef } = useDroppable({
        id: 'diagramEditor'
    })

    const [isConnectionBuilderOpen, setIsConnectionBuilderOpen] = useState(false)
    const [connectingType, setConnectingType] = useState<Type | TypeInput | null>(null)
    const [connectingPieceId, setConnectingPieceId] = useState<string | null>(null)
    const [connectedType, setConnectedType] = useState<Type | TypeInput | null>(null)
    const [connectedPieceId, setConnectedPieceId] = useState<string | null>(null)
    const [connection, setConnection] = useState<ConnectionInput | null>(null)

    const zoomToFit = () => {
        if (graphViewRef.current) {
            graphViewRef.current.handleZoomToFit()
        }
    }

    useImperativeHandle(ref, () => ({
        onDropPiece,
        onDropDesignSnippet,
        zoomToFit
    }))

    const showConnectionBuilder = () => {
        setIsConnectionBuilderOpen(true)
    }

    const handleConnectionBuilderFinished = () => {
        dispatch(
            updateDesign({
                id: designView.id,
                design: {
                    ...designRef.current,
                    connections: [
                        ...designRef.current.connections,
                        {
                            ...connection,
                            connecting: {
                                piece: {
                                    ...connection.connecting.piece,
                                    id: connectingPieceId
                                }
                            },
                            connected: {
                                piece: {
                                    ...connection.connected.piece,
                                    id: connectedPieceId
                                }
                            }
                        }
                    ]
                } as DesignInput
            })
        )
        setConnectingType(null)
        setConnectingPieceId(null)
        setConnectedType(null)
        setConnectedPieceId(null)
        setConnection(null)
        setIsConnectionBuilderOpen(false)
    }

    const handleConnectionBuilderCanceled = () => {
        setConnectingType(null)
        setConnectingPieceId(null)
        setConnectedType(null)
        setConnectedPieceId(null)
        setConnection(null)
        setIsConnectionBuilderOpen(false)
    }

    const onSelect = (newSelection: SelectionT, event?: any): void => {
        if (event == null && !newSelection.nodes && !newSelection.edges) {
            dispatch(updateDesignSelection(designViewId, [], []))
            return
        }
        // Remove the previously selected pieces and connections from the selectionState if they are in the new selection.
        // Add the new selected pieces and connections if they were not in the previous selection.
        const selectedPiecesIds = designView.selection.piecesIds.slice()
        if (newSelection.nodes) {
            newSelection.nodes.forEach((node) => {
                if (designView.selection.piecesIds.includes(node.id)) {
                    selectedPiecesIds.splice(selectedPiecesIds.indexOf(node.id), 1)
                } else {
                    selectedPiecesIds.push(node.id)
                }
            })
        }
        const selectedConnectionsIds = designView.selection.connectionsPiecesIds.slice()
        if (newSelection.edges) {
            newSelection.edges.forEach((edge) => {
                if (
                    designView.selection.connectionsPiecesIds.some(
                        ([source, target]) => source === edge.source && target === edge.target
                    )
                ) {
                    selectedConnectionsIds.splice(
                        selectedConnectionsIds.findIndex(
                            ([source, target]) => source === edge.source && target === edge.target
                        ),
                        1
                    )
                } else {
                    selectedConnectionsIds.push([edge.source, edge.target])
                }
            })
        }
        dispatch(updateDesignSelection(designViewId, selectedPiecesIds, selectedConnectionsIds))
    }

    const onCreateNode = (x: number, y: number, event: any): void => {
        dispatch(
            updateDesign({
                id: designView.id,
                design: {
                    ...designRef.current,
                    pieces: [
                        ...designRef.current.pieces,
                        {
                            id: Generator.generateRandomId(x - y),
                            type: event.piece.type,
                            root: {
                                plane: {
                                    origin: { x: 0, y: 0, z: 0 },
                                    xAxis: { x: 1, y: 0, z: 0 },
                                    yAxis: { x: 0, y: 1, z: 0 }
                                }
                            },
                            diagram: {
                                point: { x: Math.round(x), y: Math.round(y) }
                            }
                        } as PieceInput
                    ]
                }
            })
        )
    }

    const onUpdateNode = (
        node: INode,
        updatedNodes?: Map<string, INode> | null,
        updatedNodePosition?: IPoint
    ): void | Promise<any> => {
        const piece = designRef.current.pieces.find((p) => p.id === node.id)
        if (piece) {
            dispatch(
                updateDesign({
                    id: designView.id,
                    design: {
                        ...designRef.current,
                        pieces: designRef.current.pieces.map((p) =>
                            p.id === node.id
                                ? {
                                    ...p,
                                    diagram: {
                                        point: {
                                            x: Math.round(updatedNodePosition?.x ?? node.x),
                                            y: Math.round(updatedNodePosition?.y ?? node.y)
                                        }
                                    }
                                }
                                : p
                        )
                    } as DesignInput
                })
            )
        }
    }

    const onCreateEdge = (sourceNode: INode, targetNode: INode): void => {
        const connectingPieceType = types
            .get(sourceNode.piece.type.name)
            .get(sourceNode.piece.type.variant ?? '')
        const connectedPieceType = types
            .get(targetNode.piece.type.name)
            .get(targetNode.piece.type.variant ?? '')
        setConnectingType(connectingPieceType)
        setConnectingPieceId(sourceNode.id)
        setConnectedType(connectedPieceType)
        setConnectedPieceId(targetNode.id)
        showConnectionBuilder()
    }

    const onDeleteSelected = (selected: SelectionT) => {
        dispatch(
            updateDesignSelection(
                designView.id,
                designView.selection.piecesIds.filter((id) => !selected.nodes?.has(id))
            )
        )
        dispatch(
            updateDesign({
                id: designView.id,
                design: {
                    ...designRef.current,
                    pieces: designRef.current.pieces.filter(
                        (piece) => !selected.nodes?.has(piece.id)
                    ),
                    connections: designRef.current.connections.filter(
                        (connection) =>
                            !selected.edges?.has(
                                `${connection.connecting.piece.id}_${connection.connected.piece.id}`
                            ) &&
                            !selected.nodes?.has(connection.connecting.piece.id) &&
                            !selected.nodes?.has(connection.connected.piece.id)
                    )
                } as DesignInput
            })
        )
    }

    const onCopySelected = () => {
        if (selected && selected.nodes) {
            const nodesToCopy = graph.nodes.filter((node) => selected.nodes?.has(node.id))
            const toppestNode = nodesToCopy.reduce((prev, curr) => (prev.y < curr.y ? prev : curr))
            const leftestNode = nodesToCopy.reduce((prev, curr) => (prev.x < curr.x ? prev : curr))
            const edgesToCopy = graph.edges.filter((edge) =>
                selected.edges?.has(`${edge.source}_${edge.target}`)
            )
            const designSnippetToCopy = {
                pieces: nodesToCopy.map((node) => ({
                    ...node.piece,
                    diagram: {
                        point: {
                            x: Math.round(node.x - leftestNode.x),
                            y: Math.round(node.y - toppestNode.y)
                        }
                    }
                })),
                connections: edgesToCopy.map((edge) => edge.connection)
            }
            navigator.clipboard
                .writeText(JSON.stringify(designSnippetToCopy))
                .then(() => { })
                .catch((err) => { })
        }
    }

    const onPasteSelected = (selected?: SelectionT | null, xyCoords?: IPoint): void => {
        navigator.clipboard.readText().then((text) => {
            const designSnippet = JSON.parse(text)
            const oldPieceToNewPiece = new Map<string, string>()
            const placedDesignSnippet = {
                pieces: designSnippet.pieces.map((piece) => {
                    const x = Math.round(xyCoords?.x + piece.diagram.point.x)
                    const y = Math.round(xyCoords?.y + piece.diagram.point.y)
                    const id = piece.id + SEPARATOR + Generator.generateRandomId(x - y)
                    oldPieceToNewPiece.set(piece.id, id)
                    return {
                        ...piece,
                        id: id,
                        diagram: {
                            point: { x, y }
                        }
                    }
                }),
                connections: designSnippet.connections.map((connection) => ({
                    ...connection,
                    connecting: {
                        ...connection.connecting,
                        piece: {
                            ...connection.connecting.piece,
                            id: oldPieceToNewPiece.get(connection.connecting.piece.id)
                        }
                    },
                    connected: {
                        ...connection.connected,
                        piece: {
                            ...connection.connected.piece,
                            id: oldPieceToNewPiece.get(connection.connected.piece.id)
                        }
                    }
                }))
            }
            const newPieceIds = placedDesignSnippet.pieces.map((piece) => piece.id)
            if (newPieceIds.length !== new Set(newPieceIds).size) {
                message.error('All pieces must have unique ids.')
                return
            }
            dispatch(
                updateDesign({
                    id: designView.id,
                    design: {
                        ...designRef.current,
                        pieces: [...designRef.current.pieces, ...placedDesignSnippet.pieces],
                        connections: [
                            ...designRef.current.connections,
                            ...placedDesignSnippet.connections
                        ]
                    } as DesignInput
                } as DesignView)
            )
        })
    }

    const onSwapEdge = (sourceNode: INode, targetNode: INode, edge: IEdge): void => {
        console.log('onSwapEdge should not be possible', sourceNode, targetNode, edge)
        return
    }

    const canSwapEdge = (
        sourceNode: INode,
        hoveredNode: INode | null,
        swapEdge: IEdge
    ): boolean => {
        return false
    }

    const onContextMenu = (x: number, y: number, event: any): void => { }

    const renderNodeText = (
        data: IPieceNode,
        id: string | number,
        isSelected: boolean
    ): SVGProps<SVGGElement> => {
        const type = types.get(data.piece.type.name)?.get(data.piece.type.variant ?? '')
        return (
            <foreignObject x="-19" y="-19" width="38" height="38">
                <ConfigProvider locale={enUS} theme={sketchpadTheme}>
                    <ArtifactAvatar
                        icon={type.icon}
                        // description={
                        //     <>
                        //         {type?.variant ? `${type.name} - ${type.variant}` : type?.name}
                        //         <br />
                        //         {type?.description}
                        //     </>
                        // }
                        isSelected={isSelected}
                    />
                </ConfigProvider>
            </foreignObject>
        )
    }

    // Adaptation of: https://github.com/uber/react-digraph/issues/179
    const onDropPiece = (x: number, y: number, type: Type) => {
        if (graphViewRef.current) {
            const viewTransfrom = graphViewRef.current.state.viewTransform
            const svgX = -1 * ((viewTransfrom.x - x) / viewTransfrom.k)
            const svgY = -1 * ((viewTransfrom.y - y) / viewTransfrom.k)
            onCreateNode(svgX, svgY, {
                piece: {
                    type: {
                        name: type.name,
                        variant: type.variant
                    }
                } as PieceInput
            })
        }
    }

    const onDropDesignSnippet = (
        designSnippet: Design | DesignInput,
        x?: number | undefined,
        y?: number | undefined,
        existingDesign?: Design | DesignInput | null | undefined
    ) => {
        if (graphViewRef.current) {
            const viewTransfrom = graphViewRef.current.state.viewTransform
            const svgX = -1 * ((viewTransfrom.x - x) / viewTransfrom.k)
            const svgY = -1 * ((viewTransfrom.y - y) / viewTransfrom.k)

            const areAllIdsUnique = designSnippet.pieces.every((piece) => {
                return !(
                    existingDesign?.pieces.some((existingPiece) => existingPiece.id === piece.id) ??
                    true
                )
            })
            const idMap = new Map<string, string>()
            const newDesignPieces = designSnippet.pieces.map((piece) => {
                const x = Math.round(svgX + piece.diagram.point.x)
                const y = Math.round(svgY + piece.diagram.point.y)
                const id = areAllIdsUnique
                    ? piece.id
                    : piece.id + SEPARATOR + Generator.generateRandomId(x - y)
                idMap.set(piece.id, id)
                return {
                    ...piece,
                    id,
                    diagram: {
                        point: {
                            x: x,
                            y: y
                        }
                    }
                }
            })
            const newDesignConnections = designSnippet.connections.map((connection) => ({
                ...connection,
                connected: {
                    ...connection.connected,
                    piece: {
                        ...connection.connected.piece,
                        id: idMap.get(connection.connected.piece.id)
                    }
                },
                connecting: {
                    ...connection.connecting,
                    piece: {
                        ...connection.connecting.piece,
                        id: idMap.get(connection.connecting.piece.id)
                    }
                }
            }))
            dispatch(
                updateDesign({
                    id: designView.id,
                    design: {
                        ...designRef.current,
                        pieces: [...designRef.current.pieces, ...newDesignPieces],
                        connections: [...designRef.current.connections, ...newDesignConnections]
                    }
                })
            )
        }
    }

    const NodeTypes = GraphConfig.NodeTypes
    const NodeSubtypes = GraphConfig.NodeSubtypes
    const EdgeTypes = GraphConfig.EdgeTypes

    return (
        <>
            <div
                id="design-editor"
                className={
                    'font-sans h-full ' + props.className + (isOver ? 'bg-dark' : 'bg-darkGrey')
                }
                ref={setNodeRef}
            >
                <GraphView
                    ref={graphViewRef}
                    nodeKey={NODE_KEY}
                    nodes={nodes}
                    edges={edges}
                    selected={selected}
                    nodeTypes={NodeTypes}
                    nodeSubtypes={NodeSubtypes}
                    edgeTypes={EdgeTypes}
                    // layoutEngineType='HorizontalTree'
                    allowMultiselect={true}
                    // gridSpacing={20}
                    gridDotSize={0}
                    nodeSize={100}
                    edgeHandleSize={200}
                    edgeArrowSize={0.01}
                    // rotateEdgeHandle={false}
                    minZoom={0.1}
                    maxZoom={4}
                    showGraphControls={false}
                    canSwapEdge={canSwapEdge}
                    onSwapEdge={onSwapEdge}
                    onArrowClicked={(selectedEdge: IEdge): void => { }}
                    onSelect={onSelect}
                    onCreateNode={onCreateNode}
                    onUpdateNode={onUpdateNode}
                    onCreateEdge={onCreateEdge}
                    onDeleteSelected={onDeleteSelected}
                    onCopySelected={onCopySelected}
                    onPasteSelected={onPasteSelected}
                    onContextMenu={onContextMenu}
                    renderNodeText={renderNodeText}
                ></GraphView>
            </div>
            <Modal
                width={1200}
                title="New connection"
                open={isConnectionBuilderOpen}
                onOk={handleConnectionBuilderFinished}
                onCancel={handleConnectionBuilderCanceled}
                mask={false}
            >
                {connectingType && connectedType ? (
                    <ConnectionBuilder
                        connectingType={connectingType}
                        connectedType={connectedType}
                        onConnectionChange={setConnection}
                    />
                ) : null}
            </Modal>
        </>
    )
})

DiagramEditor.displayName = 'DiagramEditor'

// 3D Editor

interface IEditorContext {
    designViewId: string
    kitDirectory: string
    blobUrls: { [key: string]: string }
}

const EditorContext = createContext<IEditorContext>({} as IEditorContext)

interface PieceThreeProps {
    piece: PieceInput
    selected: boolean
}

const PieceThree = ({ piece, selected }: PieceThreeProps) => {
    const { designViewId, kitDirectory } = useContext(EditorContext)
    const dispatch = useDispatch()
    const designView = useSelector((state: RootState) => selectDesignView(state, designViewId))
    const type = useSelector((state: RootState) =>
        selectType(state, kitDirectory, piece.type.name, piece.type.variant ?? '')
    )
    const isSelected = designView.selection.piecesIds.includes(piece.id)
    return (
        <ThreeSelect
            multiple
            box
            border="1px solid #fff"
            // onChange={(selected): void => {
            //         console.log('selection starting', selected)
            //     }}
            // onChangePointerUp={(e) => {
            //     if (isSelectionBoxActive) {
            //         setIsSelectionBoxActive(false)
            //         console.log('selection ending', e)
            //     }
            // }}
            onClick={(e) => {
                const pieceId = getGroupNameFromClickEventGroupObject(e.eventObject)
                if (designView.selection.piecesIds.includes(pieceId)) {
                    dispatch(
                        updateDesignSelection(
                            designViewId,
                            designView.selection.piecesIds.filter((id) => id !== pieceId),
                            designView.selection.connectionsPiecesIds
                        )
                    )
                } else {
                    dispatch(
                        updateDesignSelection(
                            designViewId,
                            [...designView.selection.piecesIds, pieceId],
                            designView.selection.connectionsPiecesIds
                        )
                    )
                }

                e.stopPropagation()
            }}
        >
            <RepresentationThree
                id={piece.id}
                representation={type.representations.find((representation) =>
                    representation.url.endsWith('.glb')
                )}
                color={selected ? colors.primary : undefined}
            />
        </ThreeSelect>
    )
}

PieceThree.displayName = 'PieceThree'

interface HierarchyThreeProps {
    hierarchy: Hierarchy
}

const HierarchyThree = ({ hierarchy }: HierarchyThreeProps) => {
    const { designViewId } = useContext(EditorContext)
    const designView = useSelector((state: RootState) => selectDesignView(state, designViewId))
    const piece = designView.design.pieces.find((p) => p.id === hierarchy.piece.id)
    const selected = designView.selection.piecesIds.includes(piece.id)
    if (!piece) return null

    const groupRef = useRef()
    useEffect(() => {
        if (groupRef.current) {
            groupRef.current.applyMatrix4(hierarchy.transform)
        }
    }, [])

    return (
        <group name={piece.id} ref={groupRef}>
            <PieceThree piece={piece} selected={selected} />
            {hierarchy.children.map((child, i) => (
                <HierarchyThree key={i} hierarchy={child} />
            ))}
        </group>
    )
}

HierarchyThree.displayName = 'HierarchyThree'

interface DesignThreeProps {
    transformationMode?: string
}

const DesignThree = ({ transformationMode = 'translate' }: DesignThreeProps) => {
    const dispatch = useDispatch()
    const { designViewId, kitDirectory } = useContext(EditorContext)
    const designView = useSelector((state: RootState) => selectDesignView(state, designViewId))
    const ports = useSelector((state: RootState) => selectPorts(state, kitDirectory))
    if (!designView) return null
    if (!ports) return null
    const selectedHierarchyRootPiecesIds = designView.selection.piecesIds
    const hierarchies = useMemo(() => {
        return designToHierarchies(designView.design, ports)
    }, [designView.design, ports])
    const transformControlRef = useRef(null)

    return (
        <group name={designToIdString(designView.design)}>
            {hierarchies.map((hierarchy, i) =>
                selectedHierarchyRootPiecesIds.includes(hierarchy.piece.id) ? (
                    <TransformControls
                        // matrix={semioToThreeRotation().toArray()}
                        // matrixAutoUpdate={false}
                        key={i}
                        ref={transformControlRef}
                        mode={transformationMode}
                        onMouseUp={(event) => {
                            const transformControlMatrix = new Matrix4()
                            switch (transformationMode) {
                                case 'translate':
                                    transformControlMatrix.setPosition(
                                        transformControlRef.current.gap
                                    )
                                    break
                                case 'rotate':
                                    transformControlMatrix.makeRotationFromQuaternion(
                                        transformControlRef.current.tempQuaternion
                                    )
                                    break
                                default:
                                    break
                            }
                            dispatch(
                                updateDesign({
                                    id: designViewId,
                                    design: {
                                        ...designView.design,
                                        pieces: designView.design.pieces.map((piece) =>
                                            selectedHierarchyRootPiecesIds.includes(piece.id)
                                                ? {
                                                    ...piece,
                                                    root: {
                                                        plane: Plane.parse(
                                                            piece.root?.plane
                                                        ).transform(transformControlMatrix)
                                                    }
                                                }
                                                : piece
                                        )
                                    }
                                })
                            )
                        }}
                    >
                        <HierarchyThree hierarchy={hierarchy} />
                    </TransformControls>
                ) : (
                    <HierarchyThree key={i} hierarchy={hierarchy} />
                )
            )}
        </group>
    )
}

DesignThree.displayName = 'DesignThree'

interface ShapeEditorProps { }

const ShapeEditor = ({ }: ShapeEditorProps) => {
    const { designViewId } = useContext(EditorContext)
    const dispatch = useDispatch()

    const [transformationMode, setTransdesignMode] = useState('translate')

    return (
        <div className="h-full relative">
            <FloatButton.Group className="absolute right-4 top-4">
                {/* TODO: Fix hacky repositioning of icons */}
                <FloatButton
                    icon={
                        <div className="-ml-[2.5px]">
                            <OpenWithIcon />
                        </div>
                    }
                    badge={{ dot: transformationMode === 'translate', color: colors.primary }}
                    onClick={() => setTransdesignMode('translate')}
                />
                <FloatButton
                    icon={
                        <div className="-ml-[2.5px]">
                            <ThreeSixtyIcon />
                        </div>
                    }
                    badge={{ dot: transformationMode === 'rotate', color: colors.primary }}
                    onClick={() => setTransdesignMode('rotate')}
                />
            </FloatButton.Group>
            <SemioCanvas
                onPointerMissed={() => dispatch(updateDesignSelection(designViewId, [], []))}
            >
                <DesignThree transformationMode={transformationMode} />
            </SemioCanvas>
        </div>
    )
}

ShapeEditor.displayName = 'ShapeEditor'

function getItem(
    label: React.ReactNode,
    key: React.Key,
    icon?: React.ReactNode,
    children?: MenuItem[]
): MenuItem {
    return {
        key,
        icon,
        children,
        label
    } as MenuItem
}

// const SemioIcon = (props) => (
//     <svg width={48} height={48} overflow='visible' viewBox='0 0 99.95 99.921' {...props}>
//         {'-->'}
//         <g
//             style={{
//                 stroke: '#000',
//                 strokeWidth: 1,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         >
//             <g
//                 style={{
//                     fill: '#fa9500',
//                     fillOpacity: 1,
//                     stroke: '#000',
//                     strokeWidth: 1,
//                     strokeDasharray: 'none',
//                     strokeOpacity: 1
//                 }}
//             >
//                 <path
//                     fillOpacity={0}
//                     stroke='none'
//                     d='M94.789 41.727v77.939l19.984-19.985V41.727Z'
//                     style={{
//                         fill: '#fa9500',
//                         fillOpacity: 1,
//                         stroke: 'none',
//                         strokeWidth: 0.489687,
//                         strokeDasharray: 'none',
//                         strokeOpacity: 1
//                     }}
//                     transform='translate(-94.789 -19.745)'
//                 />
//             </g>
//             <g
//                 fillOpacity={0}
//                 stroke='none'
//                 style={{
//                     fill: '#ff344f',
//                     fillOpacity: 1,
//                     stroke: '#000',
//                     strokeWidth: 1,
//                     strokeDasharray: 'none',
//                     strokeOpacity: 1
//                 }}
//             >
//                 <path
//                     d='m194.71 119.666.03-98.535-19.985 19.979-.03 78.556zM94.789 19.745h98.51l-19.984 19.984H94.79Z'
//                     style={{
//                         fill: '#ff344f',
//                         fillOpacity: 1,
//                         stroke: 'none',
//                         strokeWidth: 0.489687,
//                         strokeDasharray: 'none',
//                         strokeOpacity: 1
//                     }}
//                     transform='translate(-94.789 -19.745)'
//                 />
//             </g>
//             <g
//                 fillOpacity={0}
//                 stroke='none'
//                 style={{
//                     fill: '#00a69d',
//                     fillOpacity: 1,
//                     stroke: '#000',
//                     strokeWidth: 1,
//                     strokeDasharray: 'none',
//                     strokeOpacity: 1
//                 }}
//             >
//                 <path
//                     d='m134.757 119.666 19.984-19.985h17.987v19.985zM134.757 79.697l19.984-19.984h17.987v19.984z'
//                     style={{
//                         fill: '#00a69d',
//                         fillOpacity: 1,
//                         stroke: 'none',
//                         strokeWidth: 0.489687,
//                         strokeDasharray: 'none',
//                         strokeOpacity: 1
//                     }}
//                     transform='translate(-94.789 -19.745)'
//                 />
//             </g>
//         </g>
//     </svg>
// )

// const DesignIcon = (props) => (
//     <svg width={48} height={48} {...props}>
//         <defs>
//             <marker
//                 id='a'
//                 markerHeight={0.6}
//                 markerWidth={0.6}
//                 orient='auto-start-reverse'
//                 preserveAspectRatio='xMidYMid'
//                 refX={0}
//                 refY={0}
//                 style={{
//                     overflow: 'visible'
//                 }}
//                 viewBox='0 0 1 1'
//             >
//                 <path
//                     d='m5.77 0-8.65 5V-5Z'
//                     style={{
//                         fill: colors.light,
//                         fillRule: 'evenodd',
//                         stroke: colors.light,
//                         strokeWidth: '1pt'
//                     }}
//                     transform='scale(.5)'
//                 />
//             </marker>
//         </defs>
//         <circle
//             cx={15.031}
//             cy={10.763}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//         <circle
//             cx={15.031}
//             cy={35.829}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//         <circle
//             cx={34.916}
//             cy={24}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//         <path
//             d='M15.03 30.822V17.878'
//             style={{
//                 fill: 'none',
//                 fillRule: 'evenodd',
//                 stroke: colors.light,
//                 strokeWidth: '.927333px',
//                 strokeLinecap: 'butt',
//                 strokeLinejoin: 'miter',
//                 strokeMiterlimit: 4,
//                 strokeOpacity: 1,
//                 markerEnd: 'url(#a)'
//             }}
//         />
//     </svg>
// )

// const DesignIcon = (props) => (
//     <svg width={48} height={48} {...props}>
//         <defs>
//             <marker
//                 id='a'
//                 markerHeight={0.6}
//                 markerWidth={0.6}
//                 orient='auto-start-reverse'
//                 preserveAspectRatio='xMidYMid'
//                 refX={0}
//                 refY={0}
//                 style={{
//                     overflow: 'visible'
//                 }}
//                 viewBox='0 0 1 1'
//             >
//                 <path
//                     d='m5.77 0-8.65 5V-5Z'
//                     style={{
//                         fill: colors.light,
//                         fillRule: 'evenodd',
//                         stroke: colors.light,
//                         strokeWidth: '1pt'
//                     }}
//                     transform='scale(.5)'
//                 />
//             </marker>
//         </defs>
//         <circle
//             cx={24}
//             cy={11.739}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//         <circle
//             cx={24}
//             cy={36.806}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//         <path
//             d='M24 31.799V18.855'
//             style={{
//                 fill: 'none',
//                 fillRule: 'evenodd',
//                 stroke: colors.light,
//                 strokeWidth: '.927333px',
//                 strokeLinecap: 'butt',
//                 strokeLinejoin: 'miter',
//                 strokeMiterlimit: 4,
//                 strokeOpacity: 1,
//                 markerEnd: 'url(#a)'
//             }}
//         />
//     </svg>
// )

// const TypeIcon = (props) => (
//     <svg width={48} height={48} {...props}>
//         <circle
//             cx={24}
//             cy={24}
//             r={5.007}
//             style={{
//                 fill: 'none',
//                 stroke: colors.light,
//                 strokeWidth: 0.733,
//                 strokeDasharray: 'none',
//                 strokeOpacity: 1
//             }}
//         />
//     </svg>
// )

interface DesignWindowProps {
    viewId: string
    kitDirectory: string
}

const DesignWindow = ({ viewId, kitDirectory }: DesignWindowProps): JSX.Element => {
    const designView = useSelector((state: RootState) => selectDesignView(state, viewId))
    if (!designView) return <div>Design not found</div>
    const designRef = useRef(designView.design)
    useEffect(() => {
        designRef.current = designView.design
    }, [designView.design])

    const kit = useSelector((state: RootState) => selectKit(state, kitDirectory))
    const types = useSelector((state: RootState) => selectTypes(state, kitDirectory))
    const designs = useSelector((state: RootState) => selectDesigns(state, kitDirectory))

    const [blobUrls, setBlobUrls] = useState<{ [key: string]: string }>({})
    useEffect(() => {
        kit?.types.forEach((type) => {
            const representation = type.representations.find((representation) =>
                representation.url.endsWith('.glb')
            )
            if (!representation) return
            window.electron.ipcRenderer
                .invoke('get-file-buffer', representation.url, kitDirectory)
                .then(
                    (buffer) => {
                        const blob = new Blob([buffer], { type: 'model/gltf-binary' })
                        const url = URL.createObjectURL(blob)
                        useGLTF.preload(url)
                        setBlobUrls((prev) => ({ ...prev, [representation.url]: url }))
                    },
                    (error) => {
                        console.error(error)
                    }
                )
        })
    }, [kit])

    const [activeDraggedArtifactId, setActiveDraggedArtifactId] = useState('')
    const [activeDraggedArtifact, setActiveDraggedArtifact] = useState<Type | Design>()
    const [activeDraggedArtifactKind, setActiveDraggedArtifactKind] = useState('') // type, design or ''

    useEffect(() => {
        const separatorIndex = activeDraggedArtifactId.indexOf(SEPARATOR)
        const artifactKind = activeDraggedArtifactId.substring(0, separatorIndex)
        const artifactId = activeDraggedArtifactId.substring(separatorIndex + SEPARATOR.length)
        switch (artifactKind) {
            case 'type': {
                const typeNameSeparatorIndex = artifactId.indexOf(SEPARATOR)
                const typeName = artifactId.substring(0, typeNameSeparatorIndex)
                const typeVariant = artifactId.substring(typeNameSeparatorIndex + SEPARATOR.length)
                const type = types.get(typeName)?.get(typeVariant ?? '')
                setActiveDraggedArtifact(type)
                setActiveDraggedArtifactKind('type')
                break
            }
            case 'design': {
                const designNameSeparatorIndex = artifactId.indexOf(SEPARATOR)
                const designName = artifactId.substring(0, designNameSeparatorIndex)
                const designVariant = artifactId.substring(
                    designNameSeparatorIndex + SEPARATOR.length
                )
                const design = designs.get(designName)?.get(designVariant ?? '')
                setActiveDraggedArtifact(design)
                setActiveDraggedArtifactKind('design')
                break
            }
        }
    }, [activeDraggedArtifactId])

    const diagramEditorRef = useRef(null)

    const onDragStart = (event: DragStartEvent) => {
        setActiveDraggedArtifactId(event.active.id)
    }

    const onDragEnd = (event: DragEndEvent) => {
        if (event.over && event.over.id === 'diagramEditor') {
            // relative coordinates in the diagram editor
            const relativeX = event.activatorEvent.pageX + event.delta.x - event.over.rect.left
            const relativeY = event.activatorEvent.pageY + event.delta.y - event.over.rect.top
            switch (activeDraggedArtifactKind) {
                case 'type': {
                    diagramEditorRef.current.onDropPiece(relativeX, relativeY, {
                        name: activeDraggedArtifact.name,
                        variant: activeDraggedArtifact?.variant ?? ''
                    })
                    break
                }
                case 'design': {
                    const designToDrop = designs
                        .get(activeDraggedArtifact.name)
                        ?.get(activeDraggedArtifact?.variant ?? '')
                    if (designToDrop) {
                        diagramEditorRef.current.onDropDesignSnippet(
                            designToDrop,
                            relativeX,
                            relativeY,
                            designView.design
                        )
                    }
                }
            }
        }
        setActiveDraggedArtifactId('')
    }

    if (!kit) {
        return <div>Kit not found</div>
    }

    if (!designView) {
        return <div>Design not found</div>
    }

    const actions = [
        // {
        //     id: 'reload-kit',
        //     name: 'Reload Kit',
        //     shortcut: ['$mod+r'],
        //     keywords: 'update',
        //     section: 'Files',
        //     perform: () => {
        //     }
        // },
        {
            id: 'save',
            name: 'Save',
            shortcut: ['$mod+s'],
            keywords: 'store',
            section: 'Files',
            perform: () => {
                // TODO: Inject method and remove direct ipcRenderer call
                window.electron.ipcRenderer
                    .invoke('add-local-design', kitDirectory, designRef.current)
                    .then((result) => {
                        const { design, error } = result
                        if (error) {
                            message.error(error.code)
                        } else {
                            message.success('Design saved')
                        }
                    })
            }
        }
        // {
        //     id: 'zoom-to-fit',
        //     name: 'Zoom to Fit',
        //     shortcut: ['$mod+t'],
        //     keywords: 'design',
        //     section: 'Navigation',
        //     perform: () => {
        //         if (diagramEditorRef.current) {
        //             diagramEditorRef.current.zoomToFit()
        //         }
        //     }
        // }
    ]

    return (
        <KBarProvider actions={actions}>
            <CommandBar />
            <Row className="items-center justify-between flex h-[47px] w-full bg-darkGrey border-b-thin border-lightGrey">
                <Col className="flex items-center">
                    {/* TODO: Add icons for main menu and tools */}
                </Col>
                <Col className="flex items-center">
                    <Breadcrumb>
                        <Breadcrumb.Item>{kit.name}</Breadcrumb.Item>
                        <Breadcrumb.Item>Designs</Breadcrumb.Item>
                        <Breadcrumb.Item>{designToHumanString(designView?.design)}</Breadcrumb.Item>
                    </Breadcrumb>
                </Col>
                <Col className="flex items-center">{/* TODO: Add icons for sharing, etc */}</Col>
            </Row>
            <Layout style={{ flex: 1 }}>
                <Layout>
                    <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
                        <Sider width="240px" className="border-r-thin border-lightGrey">
                            <Collapse
                                className="p-3 border-b-thin border-lightGrey font-thin uppercase"
                                defaultActiveKey={['types', 'designs']}
                                items={[
                                    {
                                        key: 'types',
                                        label: 'Types',
                                        children: (
                                            <Collapse
                                                className="p-2 font-normal text-lightGrey normal-case"
                                                defaultActiveKey={Array.from(types.keys())}
                                                items={Array.from(types.entries())
                                                    .sort()
                                                    .map(([typeName, typeVariants], index) => ({
                                                        key: typeName,
                                                        label: typeName,
                                                        children: (
                                                            <Space
                                                                className="h-auto overflow-auto grid grid-cols-[auto-fill] min-w-[40px] auto-rows-[40px] p-1"
                                                                direction="vertical"
                                                                size={10}
                                                                style={{
                                                                    gridTemplateColumns:
                                                                        'repeat(auto-fill, minmax(40px, 1fr))',
                                                                    gridAutoRows: '40px'
                                                                }}
                                                            >
                                                                {Array.from(typeVariants.entries())
                                                                    .sort()
                                                                    .map(
                                                                        (
                                                                            [typeVariant, type],
                                                                            index
                                                                        ) => (
                                                                            <ArtifactAvatar
                                                                                key={
                                                                                    'type' +
                                                                                    SEPARATOR +
                                                                                    typeName +
                                                                                    SEPARATOR +
                                                                                    typeVariant
                                                                                }
                                                                                icon={type.icon}
                                                                                description={
                                                                                    typeVariant ? (
                                                                                        <>
                                                                                            {`Variant: ${typeVariant}`}
                                                                                            <br />
                                                                                            {
                                                                                                type.description
                                                                                            }
                                                                                        </>
                                                                                    ) : (
                                                                                        type.description
                                                                                    )
                                                                                }
                                                                                draggableId={
                                                                                    'type' +
                                                                                    SEPARATOR +
                                                                                    typeName +
                                                                                    SEPARATOR +
                                                                                    typeVariant
                                                                                }
                                                                            ></ArtifactAvatar>
                                                                        )
                                                                    )}
                                                            </Space>
                                                        )
                                                    }))}
                                            />
                                        )
                                    },
                                    {
                                        key: 'designs',
                                        label: 'Designs',
                                        children: (
                                            <Collapse
                                                className="p-2 font-normal text-lightGrey normal-case"
                                                defaultActiveKey={Array.from(designs.keys())}
                                                items={Array.from(designs.entries())
                                                    .sort()
                                                    .map(([designName, designVariants], index) => ({
                                                        key: designName,
                                                        label: designName,
                                                        children: (
                                                            <Space
                                                                className="h-auto overflow-auto grid grid-cols-[auto-fill] min-w-[40px] auto-rows-[40px] p-1"
                                                                direction="vertical"
                                                                size={10}
                                                                style={{
                                                                    gridTemplateColumns:
                                                                        'repeat(auto-fill, minmax(40px, 1fr))',
                                                                    gridAutoRows: '40px'
                                                                }}
                                                            >
                                                                {Array.from(
                                                                    designVariants.entries()
                                                                )
                                                                    .sort()
                                                                    .map(
                                                                        (
                                                                            [designVariant, design],
                                                                            index
                                                                        ) => (
                                                                            <ArtifactAvatar
                                                                                key={
                                                                                    'design' +
                                                                                    SEPARATOR +
                                                                                    designName +
                                                                                    SEPARATOR +
                                                                                    designVariant
                                                                                }
                                                                                draggableId={
                                                                                    'design' +
                                                                                    SEPARATOR +
                                                                                    designName +
                                                                                    SEPARATOR +
                                                                                    designVariant
                                                                                }
                                                                                icon={design.icon}
                                                                                description={
                                                                                    designVariant ? (
                                                                                        <>
                                                                                            {`Variant: ${designVariant}`}
                                                                                            <br />
                                                                                            {
                                                                                                design.description
                                                                                            }
                                                                                        </>
                                                                                    ) : (
                                                                                        design.description
                                                                                    )
                                                                                }
                                                                            ></ArtifactAvatar>
                                                                        )
                                                                    )}
                                                            </Space>
                                                        )
                                                    }))}
                                            />
                                        )
                                    }
                                ]}
                            />
                        </Sider>
                        <EditorContext.Provider
                            value={{ kitDirectory, designViewId: viewId, blobUrls }}
                        >
                            <Content>
                                <DiagramEditor ref={diagramEditorRef} />
                            </Content>
                            <Divider className="h-full top-0" type="vertical" />
                            <Content>
                                <ShapeEditor />
                            </Content>
                            {createPortal(
                                <DragOverlay>
                                    {activeDraggedArtifactId && (
                                        <ArtifactAvatar
                                            draggableId={activeDraggedArtifactId}
                                            icon={activeDraggedArtifact?.icon ?? ''}
                                        />
                                    )}
                                </DragOverlay>,
                                document.body
                            )}
                        </EditorContext.Provider>
                    </DndContext>
                </Layout>
                <Sider className="border-l-thin border-lightGrey" width="240">
                    <Collapse
                        className="p-3 border-b-thin border-lightGrey font-thin uppercase"
                        defaultActiveKey={['scene']}
                        items={[
                            {
                                key: 'scene',
                                label: 'Scene',
                                children: (
                                    <Flex
                                        vertical={true}
                                        className="p-2 font-normal text-lightGrey normal-case"
                                    >
                                        <Label className="p-0">Level of Details</Label>
                                        <Select
                                            className="p-1"
                                            mode="multiple"
                                            allowClear
                                            placeholder="Select"
                                            options={[
                                                {
                                                    label: '1to500',
                                                    value: '1to500'
                                                },
                                                {
                                                    label: '1to200',
                                                    value: '1to200'
                                                }
                                            ]}
                                        />
                                        <Label className="p-0">Tags</Label>
                                        <Select
                                            className="p-1"
                                            mode="multiple"
                                            allowClear
                                            placeholder="Select"
                                            options={[
                                                {
                                                    label: 'volume',
                                                    value: 'volume'
                                                },
                                                {
                                                    label: 'floor plan',
                                                    value: 'floor plan'
                                                }
                                            ]}
                                        />
                                    </Flex>
                                )
                            },
                            {
                                key: 'properties',
                                label: 'Properties',
                                children: (
                                    <Flex
                                        vertical={true}
                                        className="p-2 text-lightGrey normal-case"
                                    >
                                        <div className="p-0">
                                            This will change based on the Selection.
                                        </div>
                                    </Flex>
                                )
                            }
                        ]}
                    />
                </Sider>
            </Layout>
            {/* <Footer className='p-0'>
                    <div style={{ height: '25px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <div className='flex items-center'>
                        </div>
                    </div>
                </Footer> */}
        </KBarProvider>
    )
}

interface ArtifactWizardProps {
    onOpenDirectory: () => Promise<string>
    onOpenFile: () => Promise<string>
    onFinish: () => FormProps<IArtifactView>['onFinish']
}

const ArtifactWizard = ({
    onOpenDirectory,
    onOpenFile,
    onFinish
}: ArtifactWizardProps): JSX.Element => {
    const [form] = Form.useForm()
    const kitDirectory = Form.useWatch('kitDirectory', form)

    const [onOpenDirectoryStatus, setOnOpenDirectoryStatus] = useState('idle') // idle, loading
    const [onOpenFileStatus, setOnOpenFileStatus] = useState('idle') // idle, loading

    const onOpenDirectoryFromButton = async () => {
        setOnOpenDirectoryStatus('loading')
        const selectedKitDirectory = await onOpenDirectory()
        form.setFieldsValue({
            kitDirectory: selectedKitDirectory
        })
        setOnOpenDirectoryStatus('idle')
    }
    const onOpenFileFromButton = async () => {
        setOnOpenFileStatus('loading')
        const selectedFile = await onOpenFile()
        form.setFieldsValue({
            icon: selectedFile
        })
        setOnOpenFileStatus('idle')
    }

    return (
        <Form
            className="p-3"
            form={form}
            name="Artifact Wizard"
            initialValues={{ remember: true }}
            onFinish={onFinish}
            autoComplete="off"
        >
            <Form.Item<IArtifactView>
                label="Kind"
                name="kind"
                rules={[{ required: true, message: 'What artifact do you want to create?' }]}
                initialValue={ViewKind.Design}
            >
                <Radio.Group>
                    <Radio.Button value={ViewKind.Type}>Type</Radio.Button>
                    <Radio.Button value={ViewKind.Design}>Design</Radio.Button>
                </Radio.Group>
            </Form.Item>
            <Form.Item<IArtifactView>
                label="Kit Directory"
                name="kitDirectory"
                rules={[{ required: true, message: 'In what directory is the kit?' }]}
            >
                <Button onClick={onOpenDirectoryFromButton} icon={<FolderSharpIcon />}>
                    {kitDirectory
                        ? kitDirectory
                        : onOpenDirectoryStatus === 'loading'
                            ? 'Loading...'
                            : 'Open Directory'}
                </Button>
            </Form.Item>
            <Form.Item<IArtifactView>
                label="Name"
                name="name"
                initialValue={'Untitled'}
                rules={[{ required: true, message: 'Every artifacts needs a name.' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item<IArtifactView> label="Description" name="description">
                <Input />
            </Form.Item>
            <Form.Item<IArtifactView> label="Icon" name="icon">
                <Button onClick={onOpenFileFromButton} icon={<FileUploadSharpIcon />}>
                    {onOpenFileStatus === 'loading' ? 'Loading...' : 'Upload Icon'}
                </Button>
            </Form.Item>
            <Form.Item>
                <Button htmlType="submit" className="bg-lightGrey text-dark">
                    Create
                </Button>
            </Form.Item>
        </Form>
    )
}

interface ArtifactWindowProps {
    viewId: string
    onOpenDirectory: () => Promise<string>
    onOpenFile: () => Promise<string>
}

const ArtifactWindow = ({
    viewId,
    onOpenDirectory,
    onOpenFile
}: ArtifactWindowProps): JSX.Element => {
    const dispatch = useDispatch()
    const artifactView = useSelector((state: RootState) => selectView(state, viewId))

    const onFinish: FormProps<IArtifactView>['onFinish'] = (artifactView) => {
        dispatch(loadLocalKit(artifactView.kitDirectory))
        artifactView.id = viewId
        switch (artifactView.kind) {
            case ViewKind.Type:
                dispatch(
                    addView({
                        kind: ViewKind.Type,
                        kitDirectory: artifactView.kitDirectory,
                        id: artifactView.id,
                        type: {
                            name: artifactView.name,
                            description: artifactView.description,
                            icon: artifactView.icon,
                            variant: '',
                            unit: 'm',
                            representations: [],
                            ports: [],
                            qualities: []
                        } as TypeInput
                    } as TypeView)
                )
                return
            case ViewKind.Design:
                dispatch(
                    addView({
                        kind: ViewKind.Design,
                        kitDirectory: artifactView.kitDirectory,
                        id: artifactView.id,
                        design: {
                            name: artifactView.name,
                            description: artifactView.description,
                            icon: artifactView.icon,
                            variant: '',
                            unit: 'm',
                            pieces: [],
                            connections: [],
                            qualities: []
                        } as DesignInput
                    } as DesignView)
                )
                return
            default:
                break
        }
    }

    return artifactView ? (
        <>
            {(() => {
                switch (artifactView.kind) {
                    case ViewKind.Type:
                        // return <TypeWindow viewId={viewId} kitDirectory={artifactView.kitDirectory} />
                        return <div>Soon you can create types here🥳</div>
                    case ViewKind.Design:
                        return (
                            <DesignWindow
                                viewId={viewId}
                                kitDirectory={artifactView.kitDirectory}
                            />
                        )
                    default:
                        return null
                }
            })()}
        </>
    ) : (
        <ArtifactWizard
            onFinish={onFinish}
            onOpenDirectory={onOpenDirectory}
            onOpenFile={onOpenFile}
        />
    )
}

interface AppProps {
    onWindowMinimize: () => void
    onWindowMaximize: () => void
    onWindowClose: () => void
    onOpenDirectory: () => Promise<string>
    onOpenFile: () => Promise<string>
}

const App = ({
    onWindowMinimize,
    onWindowMaximize,
    onWindowClose,
    onOpenDirectory,
    onOpenFile
}: AppProps): JSX.Element => {
    const views = useSelector((state: RootState) => selectViews(state))
    const [fullScreen, setFullScreen] = useState(false)
    const [openTabs, setOpenTabs] = useState([])
    const [activeTab, setActiveTab] = useState('home')

    return (
        <div className="h-screen w-screen">
            <ConfigProvider locale={enUS} theme={sketchpadTheme}>
                <Layout style={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
                    <Header style={{ height: 'auto' }}>
                        <div
                            style={{
                                height: '38px',
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'flex-start',
                                WebkitAppRegion: 'drag'
                            }}
                        >
                            <Tabs
                                className="p-0 flex items-center"
                                type="editable-card"
                                style={{
                                    WebkitAppRegion: 'no-drag'
                                }}
                                activeKey={activeTab}
                                onChange={(key) => setActiveTab(key)}
                                onEdit={(targetKey, action) => {
                                    if (action === 'add') {
                                        const id = nanoid()
                                        setOpenTabs([...openTabs, id])
                                        setActiveTab(id)
                                    } else if (action === 'remove') {
                                        setOpenTabs(openTabs.filter((t) => t !== targetKey))
                                        if (activeTab === targetKey) setActiveTab('home')
                                    }
                                }}
                                defaultActiveKey="home"
                                items={[
                                    {
                                        key: 'home',
                                        label: <HomeSharpIcon />,
                                        closable: false
                                    },
                                    ...openTabs.map((tab, index) => {
                                        const view = views.find((v) => v.id === tab)
                                        if (view)
                                            return {
                                                key: view?.id,
                                                label:
                                                    view.kind === ViewKind.Type
                                                        ? view.type.name +
                                                        (view.type.variant
                                                            ? ` (${view.type.variant})`
                                                            : '')
                                                        : view.design.name +
                                                        (view.design.variant
                                                            ? ` (${view.design.variant})`
                                                            : '')
                                            }
                                        return {
                                            key: tab,
                                            label: 'New Artifact'
                                        }
                                    })
                                ]}
                            />
                            <Space />
                            <div
                                style={{
                                    display: 'flex',
                                    height: '100%',
                                    justifyContent: 'flex-end',
                                    alignItems: 'center',
                                    WebkitAppRegion: 'no-drag'
                                }}
                            >
                                <Button
                                    onClick={onWindowMinimize}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    <MinimizeSharpIcon />
                                </Button>
                                <Button
                                    onClick={onWindowMaximize}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    {fullScreen ? (
                                        <FullscreenExitSharpIcon />
                                    ) : (
                                        <FullscreenSharpIcon />
                                    )}
                                </Button>
                                <Button
                                    onClick={onWindowClose}
                                    style={{
                                        height: '100%',
                                        display: 'flex',
                                        justifyContent: 'center',
                                        alignItems: 'center'
                                    }}
                                >
                                    <CloseSharpIcon />
                                </Button>
                            </div>
                        </div>
                    </Header>
                    {activeTab === 'home' ? (
                        <div className="h-full flex items-center justify-center text-lightGrey text-2xl">
                            Click + to add a new artifact.
                        </div>
                    ) : (
                        <ArtifactWindow
                            viewId={activeTab}
                            onOpenDirectory={onOpenDirectory}
                            onOpenFile={onOpenFile}
                        />
                    )}
                </Layout>
            </ConfigProvider>
        </div>
    )
}
export default App

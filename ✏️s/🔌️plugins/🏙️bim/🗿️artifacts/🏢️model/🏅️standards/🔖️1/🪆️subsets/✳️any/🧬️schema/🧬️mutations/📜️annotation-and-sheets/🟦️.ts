/** 📜️ BIM model direct mutations for annotations, views, schedules, sheets, viewports and sheet revisions. */
import type { AnnotationAnchor, AnnotationStyle, DetailLevel, Dimension, DimensionUnit, Leader, Orientation, Paper, Phase, Point2, Schedule, ScheduleCategory, ScheduleColumn, ScheduleFilter, ScheduleGroup, ScheduleSort, Sheet, SheetRevision, Tag, TagCategory, Terminator, TextNote, View, ViewCamera, ViewCategory, ViewCrop, ViewPlane, Viewport } from "../../🟦️.ts";

export interface CreateDimension {
  mutation: "createDimension";
  id: string;
  dimension: Dimension;
}

export interface DeleteDimension {
  mutation: "deleteDimension";
  id: string;
}

export interface SetDimension {
  mutation: "setDimension";
  id: string;
  anchors?: AnnotationAnchor[];
  angle?: number;
  offset?: number;
  style?: string;
  lock?: Record<string, unknown>;
  name?: string;
}

export interface CreateTag {
  mutation: "createTag";
  id: string;
  tag: Tag;
}

export interface DeleteTag {
  mutation: "deleteTag";
  id: string;
}

export interface SetTag {
  mutation: "setTag";
  id: string;
  element?: string;
  category?: TagCategory;
  offset?: Point2;
  style?: string;
}

export interface CreateTextNote {
  mutation: "createTextNote";
  id: string;
  text_note: TextNote;
}

export interface DeleteTextNote {
  mutation: "deleteTextNote";
  id: string;
}

export interface SetTextNote {
  mutation: "setTextNote";
  id: string;
  position?: Point2;
  text?: string;
  rotation?: number;
  style?: string;
}

export interface CreateLeader {
  mutation: "createLeader";
  id: string;
  leader: Leader;
}

export interface DeleteLeader {
  mutation: "deleteLeader";
  id: string;
}

export interface SetLeader {
  mutation: "setLeader";
  id: string;
  anchor?: AnnotationAnchor;
  offset?: Point2;
  text?: string;
  style?: string;
}

export interface CreateAnnotationStyle {
  mutation: "createAnnotationStyle";
  id: string;
  annotation_style: AnnotationStyle;
}

export interface DeleteAnnotationStyle {
  mutation: "deleteAnnotationStyle";
  id: string;
}

export interface SetAnnotationStyle {
  mutation: "setAnnotationStyle";
  id: string;
  name?: string;
  text_height?: number;
  terminator?: Terminator;
  unit?: DimensionUnit;
  precision?: number;
  mark_size?: number;
  gap?: number;
  overshoot?: number;
}

export interface CreateView {
  mutation: "createView";
  id: string;
  view: View;
}

export interface SetView {
  mutation: "setView";
  id: string;
  name?: string;
  storey?: string;
  plane?: ViewPlane;
  camera?: ViewCamera;
  cut_height?: Record<string, unknown>;
  depth?: number;
  crop?: ViewCrop;
  hidden?: ViewCategory[];
  phase?: Phase;
  scale?: number;
  detail?: DetailLevel;
}

export interface DeleteView {
  mutation: "deleteView";
  id: string;
}

export interface CreateSchedule {
  mutation: "createSchedule";
  id: string;
  schedule: Schedule;
}

export interface SetSchedule {
  mutation: "setSchedule";
  id: string;
  name?: string;
  category?: ScheduleCategory;
  columns?: ScheduleColumn[];
  sort?: ScheduleSort[];
  filter?: ScheduleFilter[];
  group?: ScheduleGroup[];
  itemize?: boolean;
  storeys?: string[];
  phases?: Phase[];
}

export interface DeleteSchedule {
  mutation: "deleteSchedule";
  id: string;
}

export interface CreateSheet {
  mutation: "createSheet";
  id: string;
  sheet: Sheet;
}

export interface SetSheet {
  mutation: "setSheet";
  id: string;
  number?: string;
  name?: string;
  paper?: Paper;
  orientation?: Orientation;
  project?: string;
  drawn_by?: string;
  checked_by?: string;
  date?: string;
  revision?: string;
  scale_label?: string;
}

export interface DeleteSheet {
  mutation: "deleteSheet";
  id: string;
}

export interface CreateViewport {
  mutation: "createViewport";
  id: string;
  viewport: Viewport;
}

export interface SetViewport {
  mutation: "setViewport";
  id: string;
  sheet?: string;
  view?: string;
  position?: Point2;
  scale?: number;
  crop?: ViewCrop;
  label?: Record<string, unknown>;
}

export interface DeleteViewport {
  mutation: "deleteViewport";
  id: string;
}

export interface CreateSheetRevision {
  mutation: "createSheetRevision";
  id: string;
  sheet_revision: SheetRevision;
}

export interface SetSheetRevision {
  mutation: "setSheetRevision";
  id: string;
  number?: string;
  date?: string;
  description?: string;
  author?: string;
}

export interface DeleteSheetRevision {
  mutation: "deleteSheetRevision";
  id: string;
}

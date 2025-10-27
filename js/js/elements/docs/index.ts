// #region Header

// index.ts

// 2025 Ueli Saluz

// #endregion

// Re-export components from display folder
export { Aside } from "../display/Aside";
export type { AsideProps } from "../display/Aside";
export { Card, CardGrid } from "../display/Card";
export type { CardGridProps, CardProps } from "../display/Card";

// Re-export components from docs folder
export { FileTree, FileTreeItem } from "./FileTree";
export type { FileTreeItemProps, FileTreeProps } from "./FileTree";
export { default as Page } from "./Page";
export type { PageFrontmatter, PageProps } from "./Page";
export { default as Section } from "./Section";
export type { SectionProps } from "./Section";
export { Steps } from "./Steps";
export type { StepsProps } from "./Steps";
export { TabItem, Tabs } from "./Tabs";
export type { TabItemProps, TabsProps } from "./Tabs";

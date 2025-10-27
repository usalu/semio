// #region Header

// FileTree.tsx

// 2025 Ueli Saluz

// #endregion

import {
  File,
  FileCode,
  FileJson,
  FileText,
  FileType,
  Folder,
  FolderOpen,
  Image,
  Settings,
} from "lucide-react";
import { FC, ReactNode } from "react";
import { Tree, TreeItem } from "./Tree";

// Map file extensions to appropriate icons
const getFileIcon = (fileName: string): ReactNode => {
  const extension = fileName.split(".").pop()?.toLowerCase();

  switch (extension) {
    // Code files
    case "ts":
    case "tsx":
    case "js":
    case "jsx":
    case "py":
    case "cs":
    case "java":
    case "cpp":
    case "c":
    case "h":
    case "rs":
    case "go":
      return <FileCode size={14} />;

    // JSON/Config files
    case "json":
    case "jsonc":
      return <FileJson size={14} />;

    // Text/Markdown files
    case "md":
    case "mdx":
    case "txt":
      return <FileText size={14} />;

    // Type definition files
    case "d.ts":
      return <FileType size={14} />;

    // Image files
    case "png":
    case "jpg":
    case "jpeg":
    case "gif":
    case "svg":
    case "webp":
      return <Image size={14} />;

    // Config files
    case "config":
    case "conf":
    case "yml":
    case "yaml":
    case "toml":
    case "ini":
      return <Settings size={14} />;

    // Default file icon
    default:
      return <File size={14} />;
  }
};

export interface FileTreeNode {
  name: string;
  type: "file" | "folder";
  children?: FileTreeNode[];
  path?: string;
  onClick?: () => void;
  isSelected?: boolean;
  defaultOpen?: boolean;
}

export interface FileTreeProps {
  nodes: FileTreeNode[];
  className?: string;
  showLines?: boolean;
}

const FileTreeNodeComponent: FC<{
  node: FileTreeNode;
  isLast?: boolean;
}> = ({ node, isLast = false }) => {
  const icon =
    node.type === "folder" ? <Folder size={14} /> : getFileIcon(node.name);

  if (node.type === "folder" && node.children && node.children.length > 0) {
    return (
      <TreeItem
        label={node.name}
        icon={icon}
        isSelected={node.isSelected}
        defaultOpen={node.defaultOpen ?? false}
        isLastItem={isLast}
        onClick={node.onClick}
      >
        {node.children.map((child, index) => (
          <FileTreeNodeComponent
            key={`${child.name}-${index}`}
            node={child}
            isLast={index === node.children!.length - 1}
          />
        ))}
      </TreeItem>
    );
  }

  return (
    <TreeItem
      label={node.name}
      icon={icon}
      isSelected={node.isSelected}
      isLastItem={isLast}
      onClick={node.onClick}
    />
  );
};

export const FileTree: FC<FileTreeProps> = ({
  nodes,
  className = "",
  showLines = true,
}) => {
  return (
    <Tree className={className} showLines={showLines}>
      {nodes.map((node, index) => (
        <FileTreeNodeComponent
          key={`${node.name}-${index}`}
          node={node}
          isLast={index === nodes.length - 1}
        />
      ))}
    </Tree>
  );
};

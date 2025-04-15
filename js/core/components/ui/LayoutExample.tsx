import React from 'react';
import { useLayout } from './Sketchpad';
import { Layout } from './Sketchpad';

interface LayoutExampleProps {
  title: string;
  children?: React.ReactNode;
}

/**
 * Example component that adapts its styling based on the current layout setting
 */
export const LayoutExample: React.FC<LayoutExampleProps> = ({ title, children }) => {
  const { layout } = useLayout();
  const isCompact = layout === Layout.COMPACT;
  
  return (
    <div className={`
      border rounded-md 
      ${isCompact ? 'p-2 gap-2' : 'p-4 gap-4'}
      bg-background-level-1
    `}>
      <h3 className={`
        font-semibold
        ${isCompact ? 'text-sm mb-1' : 'text-base mb-2'}
      `}>
        {title} <span className="text-muted-foreground">({layout})</span>
      </h3>
      <div className={`${isCompact ? 'space-y-1' : 'space-y-3'}`}>
        {children}
      </div>
    </div>
  );
};

/**
 * A card component that adjusts its styling based on the current layout
 */
export const LayoutCard: React.FC<{ label: string }> = ({ label }) => {
  const { layout } = useLayout();
  const isCompact = layout === Layout.COMPACT;
  
  return (
    <div className={`
      border rounded
      ${isCompact ? 'p-1.5 text-xs' : 'p-3 text-sm'}
      bg-card text-card-foreground
    `}>
      {label}
    </div>
  );
};
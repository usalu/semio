import { ExternalLink, Maximize2, Minimize2, X } from "lucide-react";
import { FC, ReactNode, useState } from "react";

import { ActionGroup, ActionGroupItem } from "../input/ActionGroup";

export interface WindowConfig {
  id: string;
  children: ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
  loading?: boolean;
  error?: Error | null;
  skeleton?: ReactNode;
  showControls?: boolean;
  onOpenInNewWindow?: () => void;
  onMaximize?: () => void;
  onMinimize?: () => void;
  onClose?: () => void;
  controls?: ReactNode;
}

interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

const DefaultErrorDisplay: FC<{ error: Error }> = ({ error }) => (
  <div className="flex flex-col items-center justify-center h-full w-full bg-background p-4">
    <div className="text-center space-y-2 max-w-md">
      <div className="text-4xl mb-4">⚠️</div>
      <h3 className="text-lg font-medium">Error</h3>
      <p className="text-sm text-muted-foreground">{error.message}</p>
    </div>
  </div>
);

const Window: FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true, loading = false, error = null, skeleton, showControls = false, onOpenInNewWindow, onMaximize, onMinimize, onClose, controls }) => {
  const [isMaximized, setIsMaximized] = useState(false);

  const handleMaximize = () => {
    setIsMaximized(!isMaximized);
    if (isMaximized && onMinimize) onMinimize();
    else if (!isMaximized && onMaximize) onMaximize();
  };

  if (!isVisible) return null;
  return (
    <div className={`relative h-full w-full ${className}`} onDoubleClick={onDoubleClick}>
      {(showControls || controls) && (
        <div className="absolute top-1 right-1 z-10">
          {controls || (
            <ActionGroup id={`${id}-window-controls`}>
              {onOpenInNewWindow && (
                <ActionGroupItem id={`${id}-window-controls-external`} onClick={onOpenInNewWindow}>
                  <ExternalLink />
                </ActionGroupItem>
              )}
              {(onMaximize || onMinimize) && (
                <ActionGroupItem id={`${id}-window-controls-maximize`} onClick={handleMaximize}>
                  {isMaximized ? <Minimize2 /> : <Maximize2 />}
                </ActionGroupItem>
              )}
              {onClose && (
                <ActionGroupItem id={`${id}-window-controls-close`} onClick={onClose} variant="destructive">
                  <X />
                </ActionGroupItem>
              )}
            </ActionGroup>
          )}
        </div>
      )}
      {error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}
    </div>
  );
};

export default Window;

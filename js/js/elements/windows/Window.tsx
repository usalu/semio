import { FC, ReactNode } from "react";

export interface WindowConfig {
  id: string;
  children: ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
  loading?: boolean;
  error?: Error | null;
  skeleton?: ReactNode;
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

const Window: FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true, loading = false, error = null, skeleton }) => {
  if (!isVisible) return null;
  return (
    <div className={`relative h-full w-full ${className}`} onDoubleClick={onDoubleClick}>
      {error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}
    </div>
  );
};

export default Window;

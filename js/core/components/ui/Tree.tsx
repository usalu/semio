import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible'
import { ChevronDown, ChevronRight } from 'lucide-react'
import { FC, ReactNode, useState } from 'react'

interface TreeSectionProps {
    label: string
    icon?: ReactNode
    children?: ReactNode
    defaultOpen?: boolean
    className?: string
}

interface TreeNodeProps {
    label: ReactNode
    icon?: ReactNode
    children?: ReactNode
    level?: number
    collapsible?: boolean
    defaultOpen?: boolean
    isLeaf?: boolean
    onClick?: () => void
    className?: string
    isSelected?: boolean
    isHighlighted?: boolean
}

interface TreeItemProps {
    label: ReactNode
    icon?: ReactNode
    level?: number
    onClick?: () => void
    className?: string
    isSelected?: boolean
    isHighlighted?: boolean
}

export const TreeSection: FC<TreeSectionProps> = ({ label, icon, children, defaultOpen = true, className = '' }) => {
    const [open, setOpen] = useState(defaultOpen)

    return (
        <Collapsible open={open} onOpenChange={setOpen}>
            <CollapsibleTrigger asChild>
                <div className={`flex items-center gap-2 py-1.5 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden ${className}`}>
                    {open ? (
                        <ChevronDown size={14} className="flex-shrink-0" />
                    ) : (
                        <ChevronRight size={14} className="flex-shrink-0" />
                    )}
                    {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
                    <span className="flex-1 text-sm text-muted-foreground uppercase tracking-wide truncate">{label}</span>
                </div>
            </CollapsibleTrigger>
            <CollapsibleContent>{children}</CollapsibleContent>
        </Collapsible>
    )
}

export const TreeNode: FC<TreeNodeProps> = ({
    label,
    icon,
    children,
    level = 0,
    collapsible = false,
    defaultOpen = true,
    isLeaf = false,
    onClick,
    className = '',
    isSelected = false,
    isHighlighted = false
}) => {
    const [open, setOpen] = useState(defaultOpen)
    const indentStyle = { paddingLeft: `${level * 1.25}rem` }

    const Trigger = collapsible ? CollapsibleTrigger : 'div'
    const Content = collapsible ? CollapsibleContent : 'div'

    const baseClasses = "flex items-center gap-2 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden"
    const stateClasses = `${isSelected ? 'bg-accent' : ''} ${isHighlighted ? 'bg-accent/50' : ''}`
    const triggerClasses = `${baseClasses} ${stateClasses} ${className}`

    const triggerContent = (
        <div
            className={triggerClasses}
            style={indentStyle}
            onClick={onClick}
        >
            {collapsible &&
                !isLeaf &&
                (open ? (
                    <ChevronDown size={14} className="flex-shrink-0" />
                ) : (
                    <ChevronRight size={14} className="flex-shrink-0" />
                ))}
            {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-sm font-normal truncate">{label}</span>
        </div>
    )

    if (collapsible) {
        return (
            <Collapsible open={open} onOpenChange={setOpen}>
                <Trigger asChild>{triggerContent}</Trigger>
                <Content>{children}</Content>
            </Collapsible>
        )
    } else if (isLeaf) {
        return triggerContent
    } else {
        return (
            <>
                {triggerContent}
                {children}
            </>
        )
    }
}

export const TreeItem: FC<TreeItemProps> = ({
    label,
    icon,
    level = 0,
    onClick,
    className = '',
    isSelected = false,
    isHighlighted = false
}) => {
    const indentStyle = { paddingLeft: `${level * 1.25}rem` }
    const baseClasses = "flex items-center gap-2 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden"
    const stateClasses = `${isSelected ? 'bg-accent' : ''} ${isHighlighted ? 'bg-accent/50' : ''}`
    const itemClasses = `${baseClasses} ${stateClasses} ${className}`

    return (
        <div
            className={itemClasses}
            style={indentStyle}
            onClick={onClick}
        >
            {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-sm font-normal truncate">{label}</span>
        </div>
    )
}

export const Tree: FC<{ children: ReactNode; className?: string }> = ({ children, className = '' }) => {
    return <div className={`w-full ${className}`}>{children}</div>
}

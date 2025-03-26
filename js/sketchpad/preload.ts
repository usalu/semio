import { contextBridge, ipcRenderer } from 'electron';

// Expose window control functions to renderer
contextBridge.exposeInMainWorld('windowControls', {
    minimize: () => ipcRenderer.invoke('minimize-window'),
    maximize: () => ipcRenderer.invoke('maximize-window'),
    close: () => ipcRenderer.invoke('close-window')
});

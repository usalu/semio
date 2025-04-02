
import React from 'react'
import ReactDOM from 'react-dom/client'

import './globals.css'

import { Diagram } from '@semio/js'

function App() {
    return (
        <div className="h-100 w-screen">
            <Diagram></Diagram>
        </div>
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)
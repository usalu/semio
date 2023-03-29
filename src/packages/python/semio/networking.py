from urllib.parse import urlparse

def getAddressFromBaseAndPort(base:str, port:int):
    return base + ':' + str(port)

from typing import Tuple
from urllib.parse import urlparse

# TODO unit test for:

# addresses = ['localhost','localhost:80','http://localhost','https://localhost:80','255.255.255.255','255.255.255.255:70','http://255.255.255.255','http://255.255.255.255:70','sem.io','sem.io:50','ftp://sem.io:50']
# [parseAddress(address) for address in addresses]

# to yield

# [('localhost', None),
#  ('localhost', '80'),
#  ('localhost', None),
#  ('localhost', 80),
#  ('255.255.255.255', None),
#  ('255.255.255.255', '70'),
#  ('255.255.255.255', None),
#  ('255.255.255.255', 70),
#  ('sem.io', None),
#  ('sem.io', '50'),
#  ('sem.io', 50)]

def parseAddress(address:str) -> Tuple[str, int] | Tuple[int , None]:
    """Parse an address into base address and port."""
    parsedAddress = urlparse(address)
    degenerated = False
    port = parsedAddress.port
    if not parsedAddress.netloc:
        # Test if only ADDRESS:PORT is provided like localhost:80 
        if parsedAddress.scheme and not parsedAddress.port:
            if parsedAddress.path.isdigit():
                baseAddress = parsedAddress.scheme
                port = parsedAddress.path
            else:
                degenerated = True
            
        elif parsedAddress.path:
            if ':' in parsedAddress.path:
                baseAddress = parsedAddress.path.split(':')[0]
            else:
                baseAddress = parsedAddress.path
        else:
            degenerated = True
    else:
        baseAddress = parsedAddress.hostname
    if degenerated:
        raise ValueError(f'The address {address} is not valid. Either localhost, ip or a url is requested.')
    return (baseAddress,port)